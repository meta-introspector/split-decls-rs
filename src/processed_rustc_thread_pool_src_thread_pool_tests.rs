/* FP:tests.rs-0001 */ #[cfg(test)]
/* FP:tests.rs-0002 */ 
/* FP:tests.rs-0003 */ use std::sync::atomic::{AtomicUsize, Ordering};
/* FP:tests.rs-0004 */ use std::sync::mpsc::channel;
/* FP:tests.rs-0005 */ use std::sync::{Arc, Mutex};
/* FP:tests.rs-0006 */ 
/* FP:tests.rs-0007 */ use crate::{Scope, ScopeFifo, ThreadPool, ThreadPoolBuilder, join};
/* FP:tests.rs-0008 */ 
/* FP:tests.rs-0009 */ #[test]
/* FP:tests.rs-0010 */ #[should_panic(expected = "Hello, world!")]
/* FP:tests.rs-0011 */ fn panic_propagate() {
/* FP:tests.rs-0012 */     let thread_pool = ThreadPoolBuilder::new().build().unwrap();
/* FP:tests.rs-0013 */     thread_pool.install(|| {
/* FP:tests.rs-0014 */         panic!("Hello, world!");
/* FP:tests.rs-0015 */     });
/* FP:tests.rs-0016 */ }
/* FP:tests.rs-0017 */ 
/* FP:tests.rs-0018 */ #[test]
/* FP:tests.rs-0019 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0020 */ fn workers_stop() {
/* FP:tests.rs-0021 */     let registry;
/* FP:tests.rs-0022 */ 
/* FP:tests.rs-0023 */     {
/* FP:tests.rs-0024 */         // once we exit this block, thread-pool will be dropped
/* FP:tests.rs-0025 */         let thread_pool = ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/* FP:tests.rs-0026 */         registry = thread_pool.install(|| {
/* FP:tests.rs-0027 */             // do some work on these threads
/* FP:tests.rs-0028 */             join_a_lot(22);
/* FP:tests.rs-0029 */ 
/* FP:tests.rs-0030 */             Arc::clone(&thread_pool.registry)
/* FP:tests.rs-0031 */         });
/* FP:tests.rs-0032 */         assert_eq!(registry.num_threads(), 22);
/* FP:tests.rs-0033 */     }
/* FP:tests.rs-0034 */ 
/* FP:tests.rs-0035 */     // once thread-pool is dropped, registry should terminate, which
/* FP:tests.rs-0036 */     // should lead to worker threads stopping
/* FP:tests.rs-0037 */     registry.wait_until_stopped();
/* FP:tests.rs-0038 */ }
/* FP:tests.rs-0039 */ 
/* FP:tests.rs-0040 */ fn join_a_lot(n: usize) {
/* FP:tests.rs-0041 */     if n > 0 {
/* FP:tests.rs-0042 */         join(|| join_a_lot(n - 1), || join_a_lot(n - 1));
/* FP:tests.rs-0043 */     }
/* FP:tests.rs-0044 */ }
/* FP:tests.rs-0045 */ 
/* FP:tests.rs-0046 */ #[test]
/* FP:tests.rs-0047 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0048 */ fn sleeper_stop() {
/* FP:tests.rs-0049 */     use std::{thread, time};
/* FP:tests.rs-0050 */ 
/* FP:tests.rs-0051 */     let registry;
/* FP:tests.rs-0052 */ 
/* FP:tests.rs-0053 */     {
/* FP:tests.rs-0054 */         // once we exit this block, thread-pool will be dropped
/* FP:tests.rs-0055 */         let thread_pool = ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/* FP:tests.rs-0056 */         registry = Arc::clone(&thread_pool.registry);
/* FP:tests.rs-0057 */ 
/* FP:tests.rs-0058 */         // Give time for at least some of the thread pool to fall asleep.
/* FP:tests.rs-0059 */         thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0060 */     }
/* FP:tests.rs-0061 */ 
/* FP:tests.rs-0062 */     // once thread-pool is dropped, registry should terminate, which
/* FP:tests.rs-0063 */     // should lead to worker threads stopping
/* FP:tests.rs-0064 */     registry.wait_until_stopped();
/* FP:tests.rs-0065 */ }
/* FP:tests.rs-0066 */ 
/* FP:tests.rs-0067 */ /// Creates a start/exit handler that increments an atomic counter.
/* FP:tests.rs-0068 */ fn count_handler() -> (Arc<AtomicUsize>, impl Fn(usize)) {
/* FP:tests.rs-0069 */     let count = Arc::new(AtomicUsize::new(0));
/* FP:tests.rs-0070 */     (Arc::clone(&count), move |_| {
/* FP:tests.rs-0071 */         count.fetch_add(1, Ordering::SeqCst);
/* FP:tests.rs-0072 */     })
/* FP:tests.rs-0073 */ }
/* FP:tests.rs-0074 */ 
/* FP:tests.rs-0075 */ /// Wait until a counter is no longer shared, then return its value.
/* FP:tests.rs-0076 */ fn wait_for_counter(mut counter: Arc<AtomicUsize>) -> usize {
/* FP:tests.rs-0077 */     use std::{thread, time};
/* FP:tests.rs-0078 */ 
/* FP:tests.rs-0079 */     for _ in 0..60 {
/* FP:tests.rs-0080 */         counter = match Arc::try_unwrap(counter) {
/* FP:tests.rs-0081 */             Ok(counter) => return counter.into_inner(),
/* FP:tests.rs-0082 */             Err(counter) => {
/* FP:tests.rs-0083 */                 thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0084 */                 counter
/* FP:tests.rs-0085 */             }
/* FP:tests.rs-0086 */         };
/* FP:tests.rs-0087 */     }
/* FP:tests.rs-0088 */ 
/* FP:tests.rs-0089 */     // That's too long!
/* FP:tests.rs-0090 */     panic!("Counter is still shared!");
/* FP:tests.rs-0091 */ }
/* FP:tests.rs-0092 */ 
/* FP:tests.rs-0093 */ #[test]
/* FP:tests.rs-0094 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0095 */ fn failed_thread_stack() {
/* FP:tests.rs-0096 */     // Note: we first tried to force failure with a `usize::MAX` stack, but
/* FP:tests.rs-0097 */     // macOS and Windows weren't fazed, or at least didn't fail the way we want.
/* FP:tests.rs-0098 */     // They work with `isize::MAX`, but 32-bit platforms may feasibly allocate a
/* FP:tests.rs-0099 */     // 2GB stack, so it might not fail until the second thread.
/* FP:tests.rs-0100 */     let stack_size = ::std::isize::MAX as usize;
/* FP:tests.rs-0101 */ 
/* FP:tests.rs-0102 */     let (start_count, start_handler) = count_handler();
/* FP:tests.rs-0103 */     let (exit_count, exit_handler) = count_handler();
/* FP:tests.rs-0104 */     let builder = ThreadPoolBuilder::new()
/* FP:tests.rs-0105 */         .num_threads(10)
/* FP:tests.rs-0106 */         .stack_size(stack_size)
/* FP:tests.rs-0107 */         .start_handler(start_handler)
/* FP:tests.rs-0108 */         .exit_handler(exit_handler);
/* FP:tests.rs-0109 */ 
/* FP:tests.rs-0110 */     let pool = builder.build();
/* FP:tests.rs-0111 */     assert!(pool.is_err(), "thread stack should have failed!");
/* FP:tests.rs-0112 */ 
/* FP:tests.rs-0113 */     // With such a huge stack, 64-bit will probably fail on the first thread;
/* FP:tests.rs-0114 */     // 32-bit might manage the first 2GB, but certainly fail the second.
/* FP:tests.rs-0115 */     let start_count = wait_for_counter(start_count);
/* FP:tests.rs-0116 */     assert!(start_count <= 1);
/* FP:tests.rs-0117 */     assert_eq!(start_count, wait_for_counter(exit_count));
/* FP:tests.rs-0118 */ }
/* FP:tests.rs-0119 */ 
/* FP:tests.rs-0120 */ #[test]
/* FP:tests.rs-0121 */ #[cfg_attr(not(panic = "unwind"), ignore)]
/* FP:tests.rs-0122 */ fn panic_thread_name() {
/* FP:tests.rs-0123 */     let (start_count, start_handler) = count_handler();
/* FP:tests.rs-0124 */     let (exit_count, exit_handler) = count_handler();
/* FP:tests.rs-0125 */     let builder = ThreadPoolBuilder::new()
/* FP:tests.rs-0126 */         .num_threads(10)
/* FP:tests.rs-0127 */         .start_handler(start_handler)
/* FP:tests.rs-0128 */         .exit_handler(exit_handler)
/* FP:tests.rs-0129 */         .thread_name(|i| {
/* FP:tests.rs-0130 */             if i >= 5 {
/* FP:tests.rs-0131 */                 panic!();
/* FP:tests.rs-0132 */             }
/* FP:tests.rs-0133 */             format!("panic_thread_name#{}", i)
/* FP:tests.rs-0134 */         });
/* FP:tests.rs-0135 */ 
/* FP:tests.rs-0136 */     let pool = crate::unwind::halt_unwinding(|| builder.build());
/* FP:tests.rs-0137 */     assert!(pool.is_err(), "thread-name panic should propagate!");
/* FP:tests.rs-0138 */ 
/* FP:tests.rs-0139 */     // Assuming they're created in order, threads 0 through 4 should have
/* FP:tests.rs-0140 */     // been started already, and then terminated by the panic.
/* FP:tests.rs-0141 */     assert_eq!(5, wait_for_counter(start_count));
/* FP:tests.rs-0142 */     assert_eq!(5, wait_for_counter(exit_count));
/* FP:tests.rs-0143 */ }
/* FP:tests.rs-0144 */ 
/* FP:tests.rs-0145 */ #[test]
/* FP:tests.rs-0146 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0147 */ fn self_install() {
/* FP:tests.rs-0148 */     let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0149 */ 
/* FP:tests.rs-0150 */     // If the inner `install` blocks, then nothing will actually run it!
/* FP:tests.rs-0151 */     assert!(pool.install(|| pool.install(|| true)));
/* FP:tests.rs-0152 */ }
/* FP:tests.rs-0153 */ 
/* FP:tests.rs-0154 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0155 */ #[test]
/* FP:tests.rs-0156 */ #[ignore]
/* FP:tests.rs-0157 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0158 */ fn mutual_install() {
/* FP:tests.rs-0159 */     let pool1 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0160 */     let pool2 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0161 */ 
/* FP:tests.rs-0162 */     let ok = pool1.install(|| {
/* FP:tests.rs-0163 */         // This creates a dependency from `pool1` -> `pool2`
/* FP:tests.rs-0164 */         pool2.install(|| {
/* FP:tests.rs-0165 */             // This creates a dependency from `pool2` -> `pool1`
/* FP:tests.rs-0166 */             pool1.install(|| {
/* FP:tests.rs-0167 */                 // If they blocked on inter-pool installs, there would be no
/* FP:tests.rs-0168 */                 // threads left to run this!
/* FP:tests.rs-0169 */                 true
/* FP:tests.rs-0170 */             })
/* FP:tests.rs-0171 */         })
/* FP:tests.rs-0172 */     });
/* FP:tests.rs-0173 */     assert!(ok);
/* FP:tests.rs-0174 */ }
/* FP:tests.rs-0175 */ 
/* FP:tests.rs-0176 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0177 */ #[test]
/* FP:tests.rs-0178 */ #[ignore]
/* FP:tests.rs-0179 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0180 */ fn mutual_install_sleepy() {
/* FP:tests.rs-0181 */     use std::{thread, time};
/* FP:tests.rs-0182 */ 
/* FP:tests.rs-0183 */     let pool1 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0184 */     let pool2 = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0185 */ 
/* FP:tests.rs-0186 */     let ok = pool1.install(|| {
/* FP:tests.rs-0187 */         // This creates a dependency from `pool1` -> `pool2`
/* FP:tests.rs-0188 */         pool2.install(|| {
/* FP:tests.rs-0189 */             // Give `pool1` time to fall asleep.
/* FP:tests.rs-0190 */             thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0191 */ 
/* FP:tests.rs-0192 */             // This creates a dependency from `pool2` -> `pool1`
/* FP:tests.rs-0193 */             pool1.install(|| {
/* FP:tests.rs-0194 */                 // Give `pool2` time to fall asleep.
/* FP:tests.rs-0195 */                 thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0196 */ 
/* FP:tests.rs-0197 */                 // If they blocked on inter-pool installs, there would be no
/* FP:tests.rs-0198 */                 // threads left to run this!
/* FP:tests.rs-0199 */                 true
/* FP:tests.rs-0200 */             })
/* FP:tests.rs-0201 */         })
/* FP:tests.rs-0202 */     });
/* FP:tests.rs-0203 */     assert!(ok);
/* FP:tests.rs-0204 */ }
/* FP:tests.rs-0205 */ 
/* FP:tests.rs-0206 */ #[test]
/* FP:tests.rs-0207 */ #[allow(deprecated)]
/* FP:tests.rs-0208 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0209 */ fn check_thread_pool_new() {
/* FP:tests.rs-0210 */     let pool = ThreadPool::new(crate::Configuration::new().num_threads(22)).unwrap();
/* FP:tests.rs-0211 */     assert_eq!(pool.current_num_threads(), 22);
/* FP:tests.rs-0212 */ }
/* FP:tests.rs-0213 */ 
/* FP:tests.rs-0214 */ macro_rules! test_scope_order {
/* FP:tests.rs-0215 */     ($scope:ident => $spawn:ident) => {{
/* FP:tests.rs-0216 */         let builder = ThreadPoolBuilder::new().num_threads(1);
/* FP:tests.rs-0217 */         let pool = builder.build().unwrap();
/* FP:tests.rs-0218 */         pool.install(|| {
/* FP:tests.rs-0219 */             let vec = Mutex::new(vec![]);
/* FP:tests.rs-0220 */             pool.$scope(|scope| {
/* FP:tests.rs-0221 */                 let vec = &vec;
/* FP:tests.rs-0222 */                 for i in 0..10 {
/* FP:tests.rs-0223 */                     scope.$spawn(move |_| {
/* FP:tests.rs-0224 */                         vec.lock().unwrap().push(i);
/* FP:tests.rs-0225 */                     });
/* FP:tests.rs-0226 */                 }
/* FP:tests.rs-0227 */             });
/* FP:tests.rs-0228 */             vec.into_inner().unwrap()
/* FP:tests.rs-0229 */         })
/* FP:tests.rs-0230 */     }};
/* FP:tests.rs-0231 */ }
/* FP:tests.rs-0232 */ 
/* FP:tests.rs-0233 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0234 */ #[test]
/* FP:tests.rs-0235 */ #[ignore]
/* FP:tests.rs-0236 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0237 */ fn scope_lifo_order() {
/* FP:tests.rs-0238 */     let vec = test_scope_order!(scope => spawn);
/* FP:tests.rs-0239 */     let expected: Vec<i32> = (0..10).rev().collect(); // LIFO -> reversed
/* FP:tests.rs-0240 */     assert_eq!(vec, expected);
/* FP:tests.rs-0241 */ }
/* FP:tests.rs-0242 */ 
/* FP:tests.rs-0243 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0244 */ #[test]
/* FP:tests.rs-0245 */ #[ignore]
/* FP:tests.rs-0246 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0247 */ fn scope_fifo_order() {
/* FP:tests.rs-0248 */     let vec = test_scope_order!(scope_fifo => spawn_fifo);
/* FP:tests.rs-0249 */     let expected: Vec<i32> = (0..10).collect(); // FIFO -> natural order
/* FP:tests.rs-0250 */     assert_eq!(vec, expected);
/* FP:tests.rs-0251 */ }
/* FP:tests.rs-0252 */ 
/* FP:tests.rs-0253 */ macro_rules! test_spawn_order {
/* FP:tests.rs-0254 */     ($spawn:ident) => {{
/* FP:tests.rs-0255 */         let builder = ThreadPoolBuilder::new().num_threads(1);
/* FP:tests.rs-0256 */         let pool = &builder.build().unwrap();
/* FP:tests.rs-0257 */         let (tx, rx) = channel();
/* FP:tests.rs-0258 */         pool.install(move || {
/* FP:tests.rs-0259 */             for i in 0..10 {
/* FP:tests.rs-0260 */                 let tx = tx.clone();
/* FP:tests.rs-0261 */                 pool.$spawn(move || {
/* FP:tests.rs-0262 */                     tx.send(i).unwrap();
/* FP:tests.rs-0263 */                 });
/* FP:tests.rs-0264 */             }
/* FP:tests.rs-0265 */         });
/* FP:tests.rs-0266 */         rx.iter().collect::<Vec<i32>>()
/* FP:tests.rs-0267 */     }};
/* FP:tests.rs-0268 */ }
/* FP:tests.rs-0269 */ 
/* FP:tests.rs-0270 */ #[test]
/* FP:tests.rs-0271 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0272 */ fn spawn_lifo_order() {
/* FP:tests.rs-0273 */     let vec = test_spawn_order!(spawn);
/* FP:tests.rs-0274 */     let expected: Vec<i32> = (0..10).rev().collect(); // LIFO -> reversed
/* FP:tests.rs-0275 */     assert_eq!(vec, expected);
/* FP:tests.rs-0276 */ }
/* FP:tests.rs-0277 */ 
/* FP:tests.rs-0278 */ #[test]
/* FP:tests.rs-0279 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0280 */ fn spawn_fifo_order() {
/* FP:tests.rs-0281 */     let vec = test_spawn_order!(spawn_fifo);
/* FP:tests.rs-0282 */     let expected: Vec<i32> = (0..10).collect(); // FIFO -> natural order
/* FP:tests.rs-0283 */     assert_eq!(vec, expected);
/* FP:tests.rs-0284 */ }
/* FP:tests.rs-0285 */ 
/* FP:tests.rs-0286 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0287 */ #[test]
/* FP:tests.rs-0288 */ #[ignore]
/* FP:tests.rs-0289 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0290 */ fn nested_scopes() {
/* FP:tests.rs-0291 */     // Create matching scopes for every thread pool.
/* FP:tests.rs-0292 */     fn nest<'scope, OP>(pools: &[ThreadPool], scopes: Vec<&Scope<'scope>>, op: OP)
/* FP:tests.rs-0293 */     where
/* FP:tests.rs-0294 */         OP: FnOnce(&[&Scope<'scope>]) + Send,
/* FP:tests.rs-0295 */     {
/* FP:tests.rs-0296 */         if let Some((pool, tail)) = pools.split_first() {
/* FP:tests.rs-0297 */             pool.scope(move |s| {
/* FP:tests.rs-0298 */                 // This move reduces the reference lifetimes by variance to match s,
/* FP:tests.rs-0299 */                 // but the actual scopes are still tied to the invariant 'scope.
/* FP:tests.rs-0300 */                 let mut scopes = scopes;
/* FP:tests.rs-0301 */                 scopes.push(s);
/* FP:tests.rs-0302 */                 nest(tail, scopes, op)
/* FP:tests.rs-0303 */             })
/* FP:tests.rs-0304 */         } else {
/* FP:tests.rs-0305 */             (op)(&scopes)
/* FP:tests.rs-0306 */         }
/* FP:tests.rs-0307 */     }
/* FP:tests.rs-0308 */ 
/* FP:tests.rs-0309 */     let pools: Vec<_> =
/* FP:tests.rs-0310 */         (0..10).map(|_| ThreadPoolBuilder::new().num_threads(1).build().unwrap()).collect();
/* FP:tests.rs-0311 */ 
/* FP:tests.rs-0312 */     let counter = AtomicUsize::new(0);
/* FP:tests.rs-0313 */     nest(&pools, vec![], |scopes| {
/* FP:tests.rs-0314 */         for &s in scopes {
/* FP:tests.rs-0315 */             s.spawn(|_| {
/* FP:tests.rs-0316 */                 // Our 'scope lets us borrow the counter in every pool.
/* FP:tests.rs-0317 */                 counter.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0318 */             });
/* FP:tests.rs-0319 */         }
/* FP:tests.rs-0320 */     });
/* FP:tests.rs-0321 */     assert_eq!(counter.into_inner(), pools.len());
/* FP:tests.rs-0322 */ }
/* FP:tests.rs-0323 */ 
/* FP:tests.rs-0324 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0325 */ #[test]
/* FP:tests.rs-0326 */ #[ignore]
/* FP:tests.rs-0327 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0328 */ fn nested_fifo_scopes() {
/* FP:tests.rs-0329 */     // Create matching fifo scopes for every thread pool.
/* FP:tests.rs-0330 */     fn nest<'scope, OP>(pools: &[ThreadPool], scopes: Vec<&ScopeFifo<'scope>>, op: OP)
/* FP:tests.rs-0331 */     where
/* FP:tests.rs-0332 */         OP: FnOnce(&[&ScopeFifo<'scope>]) + Send,
/* FP:tests.rs-0333 */     {
/* FP:tests.rs-0334 */         if let Some((pool, tail)) = pools.split_first() {
/* FP:tests.rs-0335 */             pool.scope_fifo(move |s| {
/* FP:tests.rs-0336 */                 // This move reduces the reference lifetimes by variance to match s,
/* FP:tests.rs-0337 */                 // but the actual scopes are still tied to the invariant 'scope.
/* FP:tests.rs-0338 */                 let mut scopes = scopes;
/* FP:tests.rs-0339 */                 scopes.push(s);
/* FP:tests.rs-0340 */                 nest(tail, scopes, op)
/* FP:tests.rs-0341 */             })
/* FP:tests.rs-0342 */         } else {
/* FP:tests.rs-0343 */             (op)(&scopes)
/* FP:tests.rs-0344 */         }
/* FP:tests.rs-0345 */     }
/* FP:tests.rs-0346 */ 
/* FP:tests.rs-0347 */     let pools: Vec<_> =
/* FP:tests.rs-0348 */         (0..10).map(|_| ThreadPoolBuilder::new().num_threads(1).build().unwrap()).collect();
/* FP:tests.rs-0349 */ 
/* FP:tests.rs-0350 */     let counter = AtomicUsize::new(0);
/* FP:tests.rs-0351 */     nest(&pools, vec![], |scopes| {
/* FP:tests.rs-0352 */         for &s in scopes {
/* FP:tests.rs-0353 */             s.spawn_fifo(|_| {
/* FP:tests.rs-0354 */                 // Our 'scope lets us borrow the counter in every pool.
/* FP:tests.rs-0355 */                 counter.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0356 */             });
/* FP:tests.rs-0357 */         }
/* FP:tests.rs-0358 */     });
/* FP:tests.rs-0359 */     assert_eq!(counter.into_inner(), pools.len());
/* FP:tests.rs-0360 */ }
/* FP:tests.rs-0361 */ 
/* FP:tests.rs-0362 */ #[test]
/* FP:tests.rs-0363 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0364 */ fn in_place_scope_no_deadlock() {
/* FP:tests.rs-0365 */     let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0366 */     let (tx, rx) = channel();
/* FP:tests.rs-0367 */     let rx_ref = &rx;
/* FP:tests.rs-0368 */     pool.in_place_scope(move |s| {
/* FP:tests.rs-0369 */         // With regular scopes this closure would never run because this scope op
/* FP:tests.rs-0370 */         // itself would block the only worker thread.
/* FP:tests.rs-0371 */         s.spawn(move |_| {
/* FP:tests.rs-0372 */             tx.send(()).unwrap();
/* FP:tests.rs-0373 */         });
/* FP:tests.rs-0374 */         rx_ref.recv().unwrap();
/* FP:tests.rs-0375 */     });
/* FP:tests.rs-0376 */ }
/* FP:tests.rs-0377 */ 
/* FP:tests.rs-0378 */ #[test]
/* FP:tests.rs-0379 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0380 */ fn in_place_scope_fifo_no_deadlock() {
/* FP:tests.rs-0381 */     let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
/* FP:tests.rs-0382 */     let (tx, rx) = channel();
/* FP:tests.rs-0383 */     let rx_ref = &rx;
/* FP:tests.rs-0384 */     pool.in_place_scope_fifo(move |s| {
/* FP:tests.rs-0385 */         // With regular scopes this closure would never run because this scope op
/* FP:tests.rs-0386 */         // itself would block the only worker thread.
/* FP:tests.rs-0387 */         s.spawn_fifo(move |_| {
/* FP:tests.rs-0388 */             tx.send(()).unwrap();
/* FP:tests.rs-0389 */         });
/* FP:tests.rs-0390 */         rx_ref.recv().unwrap();
/* FP:tests.rs-0391 */     });
/* FP:tests.rs-0392 */ }
/* FP:tests.rs-0393 */ 
/* FP:tests.rs-0394 */ #[test]
/* FP:tests.rs-0395 */ fn yield_now_to_spawn() {
/* FP:tests.rs-0396 */     let (tx, rx) = channel();
/* FP:tests.rs-0397 */ 
/* FP:tests.rs-0398 */     // Queue a regular spawn.
/* FP:tests.rs-0399 */     crate::spawn(move || tx.send(22).unwrap());
/* FP:tests.rs-0400 */ 
/* FP:tests.rs-0401 */     // The single-threaded fallback mode (for wasm etc.) won't
/* FP:tests.rs-0402 */     // get a chance to run the spawn if we never yield to it.
/* FP:tests.rs-0403 */     crate::registry::in_worker(move |_, _| {
/* FP:tests.rs-0404 */         crate::yield_now();
/* FP:tests.rs-0405 */     });
/* FP:tests.rs-0406 */ 
/* FP:tests.rs-0407 */     // The spawn **must** have started by now, but we still might have to wait
/* FP:tests.rs-0408 */     // for it to finish if a different thread stole it first.
/* FP:tests.rs-0409 */     assert_eq!(22, rx.recv().unwrap());
/* FP:tests.rs-0410 */ }
/* FP:tests.rs-0411 */ 
/* FP:tests.rs-0412 */ #[test]
/* FP:tests.rs-0413 */ fn yield_local_to_spawn() {
/* FP:tests.rs-0414 */     let (tx, rx) = channel();
/* FP:tests.rs-0415 */ 
/* FP:tests.rs-0416 */     // Queue a regular spawn.
/* FP:tests.rs-0417 */     crate::spawn(move || tx.send(22).unwrap());
/* FP:tests.rs-0418 */ 
/* FP:tests.rs-0419 */     // The single-threaded fallback mode (for wasm etc.) won't
/* FP:tests.rs-0420 */     // get a chance to run the spawn if we never yield to it.
/* FP:tests.rs-0421 */     crate::registry::in_worker(move |_, _| {
/* FP:tests.rs-0422 */         crate::yield_local();
/* FP:tests.rs-0423 */     });
/* FP:tests.rs-0424 */ 
/* FP:tests.rs-0425 */     // The spawn **must** have started by now, but we still might have to wait
/* FP:tests.rs-0426 */     // for it to finish if a different thread stole it first.
/* FP:tests.rs-0427 */     assert_eq!(22, rx.recv().unwrap());
/* FP:tests.rs-0428 */ }