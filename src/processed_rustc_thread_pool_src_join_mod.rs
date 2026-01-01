/* FP:mod.rs-0001 */ use std::sync::atomic::{AtomicBool, Ordering};
/* FP:mod.rs-0002 */ 
/* FP:mod.rs-0003 */ use crate::job::StackJob;
/* FP:mod.rs-0004 */ use crate::latch::SpinLatch;
/* FP:mod.rs-0005 */ use crate::{FnContext, registry, tlv, unwind};
/* FP:mod.rs-0006 */ 
/* FP:mod.rs-0007 */ #[cfg(test)]
/* FP:mod.rs-0009 */ 
/* FP:mod.rs-0010 */ /// Takes two closures and *potentially* runs them in parallel. It
/* FP:mod.rs-0011 */ /// returns a pair of the results from those closures.
/* FP:mod.rs-0012 */ ///
/* FP:mod.rs-0013 */ /// Conceptually, calling `join()` is similar to spawning two threads,
/* FP:mod.rs-0014 */ /// one executing each of the two closures. However, the
/* FP:mod.rs-0015 */ /// implementation is quite different and incurs very low
/* FP:mod.rs-0016 */ /// overhead. The underlying technique is called "work stealing": the
/* FP:mod.rs-0017 */ /// Rayon runtime uses a fixed pool of worker threads and attempts to
/* FP:mod.rs-0018 */ /// only execute code in parallel when there are idle CPUs to handle
/* FP:mod.rs-0019 */ /// it.
/* FP:mod.rs-0020 */ ///
/* FP:mod.rs-0021 */ /// When `join` is called from outside the thread pool, the calling
/* FP:mod.rs-0022 */ /// thread will block while the closures execute in the pool. When
/* FP:mod.rs-0023 */ /// `join` is called within the pool, the calling thread still actively
/* FP:mod.rs-0024 */ /// participates in the thread pool. It will begin by executing closure
/* FP:mod.rs-0025 */ /// A (on the current thread). While it is doing that, it will advertise
/* FP:mod.rs-0026 */ /// closure B as being available for other threads to execute. Once closure A
/* FP:mod.rs-0027 */ /// has completed, the current thread will try to execute closure B;
/* FP:mod.rs-0028 */ /// if however closure B has been stolen, then it will look for other work
/* FP:mod.rs-0029 */ /// while waiting for the thief to fully execute closure B. (This is the
/* FP:mod.rs-0030 */ /// typical work-stealing strategy).
/* FP:mod.rs-0031 */ ///
/* FP:mod.rs-0032 */ /// # Examples
/* FP:mod.rs-0033 */ ///
/* FP:mod.rs-0034 */ /// This example uses join to perform a quick-sort (note this is not a
/* FP:mod.rs-0035 */ /// particularly optimized implementation: if you **actually** want to
/* FP:mod.rs-0036 */ /// sort for real, you should prefer [the `par_sort` method] offered
/* FP:mod.rs-0037 */ /// by Rayon).
/* FP:mod.rs-0038 */ ///
/* FP:mod.rs-0039 */ /// [the `par_sort` method]: ../rayon/slice/trait.ParallelSliceMut.html#method.par_sort
/* FP:mod.rs-0040 */ ///
/* FP:mod.rs-0041 */ /// ```rust
/* FP:mod.rs-0042 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0043 */ /// let mut v = vec![5, 1, 8, 22, 0, 44];
/* FP:mod.rs-0044 */ /// quick_sort(&mut v);
/* FP:mod.rs-0045 */ /// assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);
/* FP:mod.rs-0046 */ ///
/* FP:mod.rs-0047 */ /// fn quick_sort<T:PartialOrd+Send>(v: &mut [T]) {
/* FP:mod.rs-0048 */ ///    if v.len() > 1 {
/* FP:mod.rs-0049 */ ///        let mid = partition(v);
/* FP:mod.rs-0050 */ ///        let (lo, hi) = v.split_at_mut(mid);
/* FP:mod.rs-0051 */ ///        rayon::join(|| quick_sort(lo),
/* FP:mod.rs-0052 */ ///                    || quick_sort(hi));
/* FP:mod.rs-0053 */ ///    }
/* FP:mod.rs-0054 */ /// }
/* FP:mod.rs-0055 */ ///
/* FP:mod.rs-0056 */ /// // Partition rearranges all items `<=` to the pivot
/* FP:mod.rs-0057 */ /// // item (arbitrary selected to be the last item in the slice)
/* FP:mod.rs-0058 */ /// // to the first half of the slice. It then returns the
/* FP:mod.rs-0059 */ /// // "dividing point" where the pivot is placed.
/* FP:mod.rs-0060 */ /// fn partition<T:PartialOrd+Send>(v: &mut [T]) -> usize {
/* FP:mod.rs-0061 */ ///     let pivot = v.len() - 1;
/* FP:mod.rs-0062 */ ///     let mut i = 0;
/* FP:mod.rs-0063 */ ///     for j in 0..pivot {
/* FP:mod.rs-0064 */ ///         if v[j] <= v[pivot] {
/* FP:mod.rs-0065 */ ///             v.swap(i, j);
/* FP:mod.rs-0066 */ ///             i += 1;
/* FP:mod.rs-0067 */ ///         }
/* FP:mod.rs-0068 */ ///     }
/* FP:mod.rs-0069 */ ///     v.swap(i, pivot);
/* FP:mod.rs-0070 */ ///     i
/* FP:mod.rs-0071 */ /// }
/* FP:mod.rs-0072 */ /// ```
/* FP:mod.rs-0073 */ ///
/* FP:mod.rs-0074 */ /// # Warning about blocking I/O
/* FP:mod.rs-0075 */ ///
/* FP:mod.rs-0076 */ /// The assumption is that the closures given to `join()` are
/* FP:mod.rs-0077 */ /// CPU-bound tasks that do not perform I/O or other blocking
/* FP:mod.rs-0078 */ /// operations. If you do perform I/O, and that I/O should block
/* FP:mod.rs-0079 */ /// (e.g., waiting for a network request), the overall performance may
/* FP:mod.rs-0080 */ /// be poor. Moreover, if you cause one closure to be blocked waiting
/* FP:mod.rs-0081 */ /// on another (for example, using a channel), that could lead to a
/* FP:mod.rs-0082 */ /// deadlock.
/* FP:mod.rs-0083 */ ///
/* FP:mod.rs-0084 */ /// # Panics
/* FP:mod.rs-0085 */ ///
/* FP:mod.rs-0086 */ /// No matter what happens, both closures will always be executed. If
/* FP:mod.rs-0087 */ /// a single closure panics, whether it be the first or second
/* FP:mod.rs-0088 */ /// closure, that panic will be propagated and hence `join()` will
/* FP:mod.rs-0089 */ /// panic with the same panic value. If both closures panic, `join()`
/* FP:mod.rs-0090 */ /// will panic with the panic value from the first closure.
/* FP:mod.rs-0091 */ pub fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
/* FP:mod.rs-0092 */ where
/* FP:mod.rs-0093 */     A: FnOnce() -> RA + Send,
/* FP:mod.rs-0094 */     B: FnOnce() -> RB + Send,
/* FP:mod.rs-0095 */     RA: Send,
/* FP:mod.rs-0096 */     RB: Send,
/* FP:mod.rs-0097 */ {
/* FP:mod.rs-0098 */     #[inline]
/* FP:mod.rs-0099 */     fn call<R>(f: impl FnOnce() -> R) -> impl FnOnce(FnContext) -> R {
/* FP:mod.rs-0100 */         move |_| f()
/* FP:mod.rs-0101 */     }
/* FP:mod.rs-0102 */ 
/* FP:mod.rs-0103 */     join_context(call(oper_a), call(oper_b))
/* FP:mod.rs-0104 */ }
/* FP:mod.rs-0105 */ 
/* FP:mod.rs-0106 */ /// Identical to `join`, except that the closures have a parameter
/* FP:mod.rs-0107 */ /// that provides context for the way the closure has been called,
/* FP:mod.rs-0108 */ /// especially indicating whether they're executing on a different
/* FP:mod.rs-0109 */ /// thread than where `join_context` was called. This will occur if
/* FP:mod.rs-0110 */ /// the second job is stolen by a different thread, or if
/* FP:mod.rs-0111 */ /// `join_context` was called from outside the thread pool to begin
/* FP:mod.rs-0112 */ /// with.
/* FP:mod.rs-0113 */ pub fn join_context<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
/* FP:mod.rs-0114 */ where
/* FP:mod.rs-0115 */     A: FnOnce(FnContext) -> RA + Send,
/* FP:mod.rs-0116 */     B: FnOnce(FnContext) -> RB + Send,
/* FP:mod.rs-0117 */     RA: Send,
/* FP:mod.rs-0118 */     RB: Send,
/* FP:mod.rs-0119 */ {
/* FP:mod.rs-0120 */     #[inline]
/* FP:mod.rs-0121 */     fn call_a<R>(f: impl FnOnce(FnContext) -> R, injected: bool) -> impl FnOnce() -> R {
/* FP:mod.rs-0122 */         move || f(FnContext::new(injected))
/* FP:mod.rs-0123 */     }
/* FP:mod.rs-0124 */ 
/* FP:mod.rs-0125 */     #[inline]
/* FP:mod.rs-0126 */     fn call_b<R>(f: impl FnOnce(FnContext) -> R) -> impl FnOnce(bool) -> R {
/* FP:mod.rs-0127 */         move |migrated| f(FnContext::new(migrated))
/* FP:mod.rs-0128 */     }
/* FP:mod.rs-0129 */ 
/* FP:mod.rs-0130 */     registry::in_worker(|worker_thread, injected| unsafe {
/* FP:mod.rs-0131 */         let tlv = tlv::get();
/* FP:mod.rs-0132 */         // Create virtual wrapper for task b; this all has to be
/* FP:mod.rs-0133 */         // done here so that the stack frame can keep it all live
/* FP:mod.rs-0134 */         // long enough.
/* FP:mod.rs-0135 */         let job_b_started = AtomicBool::new(false);
/* FP:mod.rs-0136 */         let job_b = StackJob::new(
/* FP:mod.rs-0137 */             tlv,
/* FP:mod.rs-0138 */             |migrated| {
/* FP:mod.rs-0139 */                 job_b_started.store(true, Ordering::Relaxed);
/* FP:mod.rs-0140 */                 call_b(oper_b)(migrated)
/* FP:mod.rs-0141 */             },
/* FP:mod.rs-0142 */             SpinLatch::new(worker_thread),
/* FP:mod.rs-0143 */         );
/* FP:mod.rs-0144 */         let job_b_ref = job_b.as_job_ref();
/* FP:mod.rs-0145 */         let job_b_id = job_b_ref.id();
/* FP:mod.rs-0146 */         worker_thread.push(job_b_ref);
/* FP:mod.rs-0147 */ 
/* FP:mod.rs-0148 */         // Execute task a; hopefully b gets stolen in the meantime.
/* FP:mod.rs-0149 */         let status_a = unwind::halt_unwinding(call_a(oper_a, injected));
/* FP:mod.rs-0150 */         worker_thread.wait_for_jobs::<_, false>(
/* FP:mod.rs-0151 */             &job_b.latch,
/* FP:mod.rs-0152 */             || job_b_started.load(Ordering::Relaxed),
/* FP:mod.rs-0153 */             |job| job.id() == job_b_id,
/* FP:mod.rs-0154 */             |job| {
/* FP:mod.rs-0155 */                 debug_assert_eq!(job.id(), job_b_id);
/* FP:mod.rs-0156 */                 job_b.run_inline(injected);
/* FP:mod.rs-0157 */             },
/* FP:mod.rs-0158 */         );
/* FP:mod.rs-0159 */ 
/* FP:mod.rs-0160 */         // Restore the TLV since we might have run some jobs overwriting it when waiting for job b.
/* FP:mod.rs-0161 */         tlv::set(tlv);
/* FP:mod.rs-0162 */ 
/* FP:mod.rs-0163 */         let result_a = match status_a {
/* FP:mod.rs-0164 */             Ok(v) => v,
/* FP:mod.rs-0165 */             Err(err) => unwind::resume_unwinding(err),
/* FP:mod.rs-0166 */         };
/* FP:mod.rs-0167 */         (result_a, job_b.into_result())
/* FP:mod.rs-0168 */     })
/* FP:mod.rs-0169 */ }