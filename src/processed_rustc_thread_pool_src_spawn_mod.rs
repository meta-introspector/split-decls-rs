/* FP:mod.rs-0001 */ use std::mem;
/* FP:mod.rs-0002 */ use std::sync::Arc;
/* FP:mod.rs-0003 */ 
/* FP:mod.rs-0004 */ use crate::job::*;
/* FP:mod.rs-0005 */ use crate::registry::Registry;
/* FP:mod.rs-0006 */ use crate::tlv::Tlv;
/* FP:mod.rs-0007 */ use crate::unwind;
/* FP:mod.rs-0008 */ 
/* FP:mod.rs-0009 */ /// Puts the task into the Rayon threadpool's job queue in the "static"
/* FP:mod.rs-0010 */ /// or "global" scope. Just like a standard thread, this task is not
/* FP:mod.rs-0011 */ /// tied to the current stack frame, and hence it cannot hold any
/* FP:mod.rs-0012 */ /// references other than those with `'static` lifetime. If you want
/* FP:mod.rs-0013 */ /// to spawn a task that references stack data, use [the `scope()`
/* FP:mod.rs-0014 */ /// function][scope] to create a scope.
/* FP:mod.rs-0015 */ ///
/* FP:mod.rs-0016 */ /// [scope]: fn.scope.html
/* FP:mod.rs-0017 */ ///
/* FP:mod.rs-0018 */ /// Since tasks spawned with this function cannot hold references into
/* FP:mod.rs-0019 */ /// the enclosing stack frame, you almost certainly want to use a
/* FP:mod.rs-0020 */ /// `move` closure as their argument (otherwise, the closure will
/* FP:mod.rs-0021 */ /// typically hold references to any variables from the enclosing
/* FP:mod.rs-0022 */ /// function that you happen to use).
/* FP:mod.rs-0023 */ ///
/* FP:mod.rs-0024 */ /// This API assumes that the closure is executed purely for its
/* FP:mod.rs-0025 */ /// side-effects (i.e., it might send messages, modify data protected
/* FP:mod.rs-0026 */ /// by a mutex, or some such thing).
/* FP:mod.rs-0027 */ ///
/* FP:mod.rs-0028 */ /// There is no guaranteed order of execution for spawns, given that
/* FP:mod.rs-0029 */ /// other threads may steal tasks at any time. However, they are
/* FP:mod.rs-0030 */ /// generally prioritized in a LIFO order on the thread from which
/* FP:mod.rs-0031 */ /// they were spawned. Other threads always steal from the other end of
/* FP:mod.rs-0032 */ /// the deque, like FIFO order. The idea is that "recent" tasks are
/* FP:mod.rs-0033 */ /// most likely to be fresh in the local CPU's cache, while other
/* FP:mod.rs-0034 */ /// threads can steal older "stale" tasks. For an alternate approach,
/* FP:mod.rs-0035 */ /// consider [`spawn_fifo()`] instead.
/* FP:mod.rs-0036 */ ///
/* FP:mod.rs-0037 */ /// [`spawn_fifo()`]: fn.spawn_fifo.html
/* FP:mod.rs-0038 */ ///
/* FP:mod.rs-0039 */ /// # Panic handling
/* FP:mod.rs-0040 */ ///
/* FP:mod.rs-0041 */ /// If this closure should panic, the resulting panic will be
/* FP:mod.rs-0042 */ /// propagated to the panic handler registered in the `ThreadPoolBuilder`,
/* FP:mod.rs-0043 */ /// if any. See [`ThreadPoolBuilder::panic_handler()`][ph] for more
/* FP:mod.rs-0044 */ /// details.
/* FP:mod.rs-0045 */ ///
/* FP:mod.rs-0046 */ /// [ph]: struct.ThreadPoolBuilder.html#method.panic_handler
/* FP:mod.rs-0047 */ ///
/* FP:mod.rs-0048 */ /// # Examples
/* FP:mod.rs-0049 */ ///
/* FP:mod.rs-0050 */ /// This code creates a Rayon task that increments a global counter.
/* FP:mod.rs-0051 */ ///
/* FP:mod.rs-0052 */ /// ```rust
/* FP:mod.rs-0053 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0054 */ /// use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
/* FP:mod.rs-0055 */ ///
/* FP:mod.rs-0056 */ /// static GLOBAL_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
/* FP:mod.rs-0057 */ ///
/* FP:mod.rs-0058 */ /// rayon::spawn(move || {
/* FP:mod.rs-0059 */ ///     GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
/* FP:mod.rs-0060 */ /// });
/* FP:mod.rs-0061 */ /// ```
/* FP:mod.rs-0062 */ pub fn spawn<F>(func: F)
/* FP:mod.rs-0063 */ where
/* FP:mod.rs-0064 */     F: FnOnce() + Send + 'static,
/* FP:mod.rs-0065 */ {
/* FP:mod.rs-0066 */     // We assert that current registry has not terminated.
/* FP:mod.rs-0067 */     unsafe { spawn_in(func, &Registry::current()) }
/* FP:mod.rs-0068 */ }
/* FP:mod.rs-0069 */ 
/* FP:mod.rs-0070 */ /// Spawns an asynchronous job in `registry.`
/* FP:mod.rs-0071 */ ///
/* FP:mod.rs-0072 */ /// Unsafe because `registry` must not yet have terminated.
/* FP:mod.rs-0073 */ pub(super) unsafe fn spawn_in<F>(func: F, registry: &Arc<Registry>)
/* FP:mod.rs-0074 */ where
/* FP:mod.rs-0075 */     F: FnOnce() + Send + 'static,
/* FP:mod.rs-0076 */ {
/* FP:mod.rs-0077 */     // We assert that this does not hold any references (we know
/* FP:mod.rs-0078 */     // this because of the `'static` bound in the interface);
/* FP:mod.rs-0079 */     // moreover, we assert that the code below is not supposed to
/* FP:mod.rs-0080 */     // be able to panic, and hence the data won't leak but will be
/* FP:mod.rs-0081 */     // enqueued into some deque for later execution.
/* FP:mod.rs-0082 */     let abort_guard = unwind::AbortIfPanic; // just in case we are wrong, and code CAN panic
/* FP:mod.rs-0083 */     let job_ref = unsafe { spawn_job(func, registry) };
/* FP:mod.rs-0084 */     registry.inject_or_push(job_ref);
/* FP:mod.rs-0085 */     mem::forget(abort_guard);
/* FP:mod.rs-0086 */ }
/* FP:mod.rs-0087 */ 
/* FP:mod.rs-0088 */ unsafe fn spawn_job<F>(func: F, registry: &Arc<Registry>) -> JobRef
/* FP:mod.rs-0089 */ where
/* FP:mod.rs-0090 */     F: FnOnce() + Send + 'static,
/* FP:mod.rs-0091 */ {
/* FP:mod.rs-0092 */     // Ensure that registry cannot terminate until this job has
/* FP:mod.rs-0093 */     // executed. This ref is decremented at the (*) below.
/* FP:mod.rs-0094 */     registry.increment_terminate_count();
/* FP:mod.rs-0095 */ 
/* FP:mod.rs-0096 */     HeapJob::new(Tlv::null(), {
/* FP:mod.rs-0097 */         let registry = Arc::clone(registry);
/* FP:mod.rs-0098 */         move |_| {
/* FP:mod.rs-0099 */             registry.catch_unwind(func);
/* FP:mod.rs-0100 */             registry.terminate(); // (*) permit registry to terminate now
/* FP:mod.rs-0101 */         }
/* FP:mod.rs-0102 */     })
/* FP:mod.rs-0103 */     .into_static_job_ref()
/* FP:mod.rs-0104 */ }
/* FP:mod.rs-0105 */ 
/* FP:mod.rs-0106 */ /// Fires off a task into the Rayon threadpool in the "static" or
/* FP:mod.rs-0107 */ /// "global" scope. Just like a standard thread, this task is not
/* FP:mod.rs-0108 */ /// tied to the current stack frame, and hence it cannot hold any
/* FP:mod.rs-0109 */ /// references other than those with `'static` lifetime. If you want
/* FP:mod.rs-0110 */ /// to spawn a task that references stack data, use [the `scope_fifo()`
/* FP:mod.rs-0111 */ /// function](fn.scope_fifo.html) to create a scope.
/* FP:mod.rs-0112 */ ///
/* FP:mod.rs-0113 */ /// The behavior is essentially the same as [the `spawn`
/* FP:mod.rs-0114 */ /// function](fn.spawn.html), except that calls from the same thread
/* FP:mod.rs-0115 */ /// will be prioritized in FIFO order. This is similar to the now-
/* FP:mod.rs-0116 */ /// deprecated [`breadth_first`] option, except the effect is isolated
/* FP:mod.rs-0117 */ /// to relative `spawn_fifo` calls, not all threadpool tasks.
/* FP:mod.rs-0118 */ ///
/* FP:mod.rs-0119 */ /// For more details on this design, see Rayon [RFC #1].
/* FP:mod.rs-0120 */ ///
/* FP:mod.rs-0121 */ /// [`breadth_first`]: struct.ThreadPoolBuilder.html#method.breadth_first
/* FP:mod.rs-0122 */ /// [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md
/* FP:mod.rs-0123 */ ///
/* FP:mod.rs-0124 */ /// # Panic handling
/* FP:mod.rs-0125 */ ///
/* FP:mod.rs-0126 */ /// If this closure should panic, the resulting panic will be
/* FP:mod.rs-0127 */ /// propagated to the panic handler registered in the `ThreadPoolBuilder`,
/* FP:mod.rs-0128 */ /// if any. See [`ThreadPoolBuilder::panic_handler()`][ph] for more
/* FP:mod.rs-0129 */ /// details.
/* FP:mod.rs-0130 */ ///
/* FP:mod.rs-0131 */ /// [ph]: struct.ThreadPoolBuilder.html#method.panic_handler
/* FP:mod.rs-0132 */ pub fn spawn_fifo<F>(func: F)
/* FP:mod.rs-0133 */ where
/* FP:mod.rs-0134 */     F: FnOnce() + Send + 'static,
/* FP:mod.rs-0135 */ {
/* FP:mod.rs-0136 */     // We assert that current registry has not terminated.
/* FP:mod.rs-0137 */     unsafe { spawn_fifo_in(func, &Registry::current()) }
/* FP:mod.rs-0138 */ }
/* FP:mod.rs-0139 */ 
/* FP:mod.rs-0140 */ /// Spawns an asynchronous FIFO job in `registry.`
/* FP:mod.rs-0141 */ ///
/* FP:mod.rs-0142 */ /// Unsafe because `registry` must not yet have terminated.
/* FP:mod.rs-0143 */ pub(super) unsafe fn spawn_fifo_in<F>(func: F, registry: &Arc<Registry>)
/* FP:mod.rs-0144 */ where
/* FP:mod.rs-0145 */     F: FnOnce() + Send + 'static,
/* FP:mod.rs-0146 */ {
/* FP:mod.rs-0147 */     // We assert that this does not hold any references (we know
/* FP:mod.rs-0148 */     // this because of the `'static` bound in the interface);
/* FP:mod.rs-0149 */     // moreover, we assert that the code below is not supposed to
/* FP:mod.rs-0150 */     // be able to panic, and hence the data won't leak but will be
/* FP:mod.rs-0151 */     // enqueued into some deque for later execution.
/* FP:mod.rs-0152 */     let abort_guard = unwind::AbortIfPanic; // just in case we are wrong, and code CAN panic
/* FP:mod.rs-0153 */     let job_ref = unsafe { spawn_job(func, registry) };
/* FP:mod.rs-0154 */ 
/* FP:mod.rs-0155 */     // If we're in the pool, use our thread's private fifo for this thread to execute
/* FP:mod.rs-0156 */     // in a locally-FIFO order. Otherwise, just use the pool's global injector.
/* FP:mod.rs-0157 */     match registry.current_thread() {
/* FP:mod.rs-0158 */         Some(worker) => unsafe { worker.push_fifo(job_ref) },
/* FP:mod.rs-0159 */         None => registry.inject(job_ref),
/* FP:mod.rs-0160 */     }
/* FP:mod.rs-0161 */     mem::forget(abort_guard);
/* FP:mod.rs-0162 */ }
/* FP:mod.rs-0163 */ 
/* FP:mod.rs-0164 */ #[cfg(test)]