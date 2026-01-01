/* FP:tests.rs-0001 */ #[cfg(test)]
/* FP:tests.rs-0002 */ 
/* FP:tests.rs-0003 */ use std::sync::atomic::{AtomicUsize, Ordering};
/* FP:tests.rs-0004 */ use std::sync::{Arc, Barrier};
/* FP:tests.rs-0005 */ 
/* FP:tests.rs-0006 */ use crate::{ThreadPoolBuildError, ThreadPoolBuilder};
/* FP:tests.rs-0007 */ 
/* FP:tests.rs-0008 */ #[test]
/* FP:tests.rs-0009 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0010 */ fn worker_thread_index() {
/* FP:tests.rs-0011 */     let pool = ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/* FP:tests.rs-0012 */     assert_eq!(pool.current_num_threads(), 22);
/* FP:tests.rs-0013 */     assert_eq!(pool.current_thread_index(), None);
/* FP:tests.rs-0014 */     let index = pool.install(|| pool.current_thread_index().unwrap());
/* FP:tests.rs-0015 */     assert!(index < 22);
/* FP:tests.rs-0016 */ }
/* FP:tests.rs-0017 */ 
/* FP:tests.rs-0018 */ #[test]
/* FP:tests.rs-0019 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0020 */ fn start_callback_called() {
/* FP:tests.rs-0021 */     let n_threads = 16;
/* FP:tests.rs-0022 */     let n_called = Arc::new(AtomicUsize::new(0));
/* FP:tests.rs-0023 */     // Wait for all the threads in the pool plus the one running tests.
/* FP:tests.rs-0024 */     let barrier = Arc::new(Barrier::new(n_threads + 1));
/* FP:tests.rs-0025 */ 
/* FP:tests.rs-0026 */     let b = Arc::clone(&barrier);
/* FP:tests.rs-0027 */     let nc = Arc::clone(&n_called);
/* FP:tests.rs-0028 */     let start_handler = move |_| {
/* FP:tests.rs-0029 */         nc.fetch_add(1, Ordering::SeqCst);
/* FP:tests.rs-0030 */         b.wait();
/* FP:tests.rs-0031 */     };
/* FP:tests.rs-0032 */ 
/* FP:tests.rs-0033 */     let conf = ThreadPoolBuilder::new().num_threads(n_threads).start_handler(start_handler);
/* FP:tests.rs-0034 */     let _ = conf.build().unwrap();
/* FP:tests.rs-0035 */ 
/* FP:tests.rs-0036 */     // Wait for all the threads to have been scheduled to run.
/* FP:tests.rs-0037 */     barrier.wait();
/* FP:tests.rs-0038 */ 
/* FP:tests.rs-0039 */     // The handler must have been called on every started thread.
/* FP:tests.rs-0040 */     assert_eq!(n_called.load(Ordering::SeqCst), n_threads);
/* FP:tests.rs-0041 */ }
/* FP:tests.rs-0042 */ 
/* FP:tests.rs-0043 */ #[test]
/* FP:tests.rs-0044 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0045 */ fn exit_callback_called() {
/* FP:tests.rs-0046 */     let n_threads = 16;
/* FP:tests.rs-0047 */     let n_called = Arc::new(AtomicUsize::new(0));
/* FP:tests.rs-0048 */     // Wait for all the threads in the pool plus the one running tests.
/* FP:tests.rs-0049 */     let barrier = Arc::new(Barrier::new(n_threads + 1));
/* FP:tests.rs-0050 */ 
/* FP:tests.rs-0051 */     let b = Arc::clone(&barrier);
/* FP:tests.rs-0052 */     let nc = Arc::clone(&n_called);
/* FP:tests.rs-0053 */     let exit_handler = move |_| {
/* FP:tests.rs-0054 */         nc.fetch_add(1, Ordering::SeqCst);
/* FP:tests.rs-0055 */         b.wait();
/* FP:tests.rs-0056 */     };
/* FP:tests.rs-0057 */ 
/* FP:tests.rs-0058 */     let conf = ThreadPoolBuilder::new().num_threads(n_threads).exit_handler(exit_handler);
/* FP:tests.rs-0059 */     {
/* FP:tests.rs-0060 */         let _ = conf.build().unwrap();
/* FP:tests.rs-0061 */         // Drop the pool so it stops the running threads.
/* FP:tests.rs-0062 */     }
/* FP:tests.rs-0063 */ 
/* FP:tests.rs-0064 */     // Wait for all the threads to have been scheduled to run.
/* FP:tests.rs-0065 */     barrier.wait();
/* FP:tests.rs-0066 */ 
/* FP:tests.rs-0067 */     // The handler must have been called on every exiting thread.
/* FP:tests.rs-0068 */     assert_eq!(n_called.load(Ordering::SeqCst), n_threads);
/* FP:tests.rs-0069 */ }
/* FP:tests.rs-0070 */ 
/* FP:tests.rs-0071 */ #[test]
/* FP:tests.rs-0072 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0073 */ fn handler_panics_handled_correctly() {
/* FP:tests.rs-0074 */     let n_threads = 16;
/* FP:tests.rs-0075 */     let n_called = Arc::new(AtomicUsize::new(0));
/* FP:tests.rs-0076 */     // Wait for all the threads in the pool plus the one running tests.
/* FP:tests.rs-0077 */     let start_barrier = Arc::new(Barrier::new(n_threads + 1));
/* FP:tests.rs-0078 */     let exit_barrier = Arc::new(Barrier::new(n_threads + 1));
/* FP:tests.rs-0079 */ 
/* FP:tests.rs-0080 */     let start_handler = move |_| {
/* FP:tests.rs-0081 */         panic!("ensure panic handler is called when starting");
/* FP:tests.rs-0082 */     };
/* FP:tests.rs-0083 */     let exit_handler = move |_| {
/* FP:tests.rs-0084 */         panic!("ensure panic handler is called when exiting");
/* FP:tests.rs-0085 */     };
/* FP:tests.rs-0086 */ 
/* FP:tests.rs-0087 */     let sb = Arc::clone(&start_barrier);
/* FP:tests.rs-0088 */     let eb = Arc::clone(&exit_barrier);
/* FP:tests.rs-0089 */     let nc = Arc::clone(&n_called);
/* FP:tests.rs-0090 */     let panic_handler = move |_| {
/* FP:tests.rs-0091 */         let val = nc.fetch_add(1, Ordering::SeqCst);
/* FP:tests.rs-0092 */         if val < n_threads {
/* FP:tests.rs-0093 */             sb.wait();
/* FP:tests.rs-0094 */         } else {
/* FP:tests.rs-0095 */             eb.wait();
/* FP:tests.rs-0096 */         }
/* FP:tests.rs-0097 */     };
/* FP:tests.rs-0098 */ 
/* FP:tests.rs-0099 */     let conf = ThreadPoolBuilder::new()
/* FP:tests.rs-0100 */         .num_threads(n_threads)
/* FP:tests.rs-0101 */         .start_handler(start_handler)
/* FP:tests.rs-0102 */         .exit_handler(exit_handler)
/* FP:tests.rs-0103 */         .panic_handler(panic_handler);
/* FP:tests.rs-0104 */     {
/* FP:tests.rs-0105 */         let _ = conf.build().unwrap();
/* FP:tests.rs-0106 */ 
/* FP:tests.rs-0107 */         // Wait for all the threads to start, panic in the start handler,
/* FP:tests.rs-0108 */         // and been taken care of by the panic handler.
/* FP:tests.rs-0109 */         start_barrier.wait();
/* FP:tests.rs-0110 */ 
/* FP:tests.rs-0111 */         // Drop the pool so it stops the running threads.
/* FP:tests.rs-0112 */     }
/* FP:tests.rs-0113 */ 
/* FP:tests.rs-0114 */     // Wait for all the threads to exit, panic in the exit handler,
/* FP:tests.rs-0115 */     // and been taken care of by the panic handler.
/* FP:tests.rs-0116 */     exit_barrier.wait();
/* FP:tests.rs-0117 */ 
/* FP:tests.rs-0118 */     // The panic handler must have been called twice on every thread.
/* FP:tests.rs-0119 */     assert_eq!(n_called.load(Ordering::SeqCst), 2 * n_threads);
/* FP:tests.rs-0120 */ }
/* FP:tests.rs-0121 */ 
/* FP:tests.rs-0122 */ #[test]
/* FP:tests.rs-0123 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0124 */ fn check_config_build() {
/* FP:tests.rs-0125 */     let pool = ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/* FP:tests.rs-0126 */     assert_eq!(pool.current_num_threads(), 22);
/* FP:tests.rs-0127 */ }
/* FP:tests.rs-0128 */ 
/* FP:tests.rs-0129 */ /// Helper used by check_error_send_sync to ensure ThreadPoolBuildError is Send + Sync
/* FP:tests.rs-0130 */ fn _send_sync<T: Send + Sync>() {}
/* FP:tests.rs-0131 */ 
/* FP:tests.rs-0132 */ #[test]
/* FP:tests.rs-0133 */ fn check_error_send_sync() {
/* FP:tests.rs-0134 */     _send_sync::<ThreadPoolBuildError>();
/* FP:tests.rs-0135 */ }
/* FP:tests.rs-0136 */ 
/* FP:tests.rs-0137 */ #[allow(deprecated)]
/* FP:tests.rs-0138 */ #[test]
/* FP:tests.rs-0139 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0140 */ fn configuration() {
/* FP:tests.rs-0141 */     let start_handler = move |_| {};
/* FP:tests.rs-0142 */     let exit_handler = move |_| {};
/* FP:tests.rs-0143 */     let panic_handler = move |_| {};
/* FP:tests.rs-0144 */     let thread_name = move |i| format!("thread_name_{}", i);
/* FP:tests.rs-0145 */ 
/* FP:tests.rs-0146 */     // Ensure we can call all public methods on Configuration
/* FP:tests.rs-0147 */     crate::Configuration::new()
/* FP:tests.rs-0148 */         .thread_name(thread_name)
/* FP:tests.rs-0149 */         .num_threads(5)
/* FP:tests.rs-0150 */         .panic_handler(panic_handler)
/* FP:tests.rs-0151 */         .stack_size(4e6 as usize)
/* FP:tests.rs-0152 */         .breadth_first()
/* FP:tests.rs-0153 */         .start_handler(start_handler)
/* FP:tests.rs-0154 */         .exit_handler(exit_handler)
/* FP:tests.rs-0155 */         .build()
/* FP:tests.rs-0156 */         .unwrap();
/* FP:tests.rs-0157 */ }
/* FP:tests.rs-0158 */ 
/* FP:tests.rs-0159 */ #[test]
/* FP:tests.rs-0160 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0161 */ fn default_pool() {
/* FP:tests.rs-0162 */     ThreadPoolBuilder::default().build().unwrap();
/* FP:tests.rs-0163 */ }
/* FP:tests.rs-0164 */ 
/* FP:tests.rs-0165 */ /// Test that custom spawned threads get their `WorkerThread` cleared once
/* FP:tests.rs-0166 */ /// the pool is done with them, allowing them to be used with rayon again
/* FP:tests.rs-0167 */ /// later. e.g. WebAssembly want to have their own pool of available threads.
/* FP:tests.rs-0168 */ #[test]
/* FP:tests.rs-0169 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0170 */ fn cleared_current_thread() -> Result<(), ThreadPoolBuildError> {
/* FP:tests.rs-0171 */     let n_threads = 5;
/* FP:tests.rs-0172 */     let mut handles = vec![];
/* FP:tests.rs-0173 */     let pool = ThreadPoolBuilder::new()
/* FP:tests.rs-0174 */         .num_threads(n_threads)
/* FP:tests.rs-0175 */         .spawn_handler(|thread| {
/* FP:tests.rs-0176 */             let handle = std::thread::spawn(move || {
/* FP:tests.rs-0177 */                 thread.run();
/* FP:tests.rs-0178 */ 
/* FP:tests.rs-0179 */                 // Afterward, the current thread shouldn't be set anymore.
/* FP:tests.rs-0180 */                 assert_eq!(crate::current_thread_index(), None);
/* FP:tests.rs-0181 */             });
/* FP:tests.rs-0182 */             handles.push(handle);
/* FP:tests.rs-0183 */             Ok(())
/* FP:tests.rs-0184 */         })
/* FP:tests.rs-0185 */         .build()?;
/* FP:tests.rs-0186 */     assert_eq!(handles.len(), n_threads);
/* FP:tests.rs-0187 */ 
/* FP:tests.rs-0188 */     pool.install(|| assert!(crate::current_thread_index().is_some()));
/* FP:tests.rs-0189 */     drop(pool);
/* FP:tests.rs-0190 */ 
/* FP:tests.rs-0191 */     // Wait for all threads to make their assertions and exit
/* FP:tests.rs-0192 */     for handle in handles {
/* FP:tests.rs-0193 */         handle.join().unwrap();
/* FP:tests.rs-0194 */     }
/* FP:tests.rs-0195 */ 
/* FP:tests.rs-0196 */     Ok(())
/* FP:tests.rs-0197 */ }