/* FP:tests.rs-0001 */ #[cfg(test)]
/* FP:tests.rs-0002 */ 
/* FP:tests.rs-0003 */ use std::sync::Arc;
/* FP:tests.rs-0004 */ use std::sync::atomic::{AtomicUsize, Ordering};
/* FP:tests.rs-0005 */ use std::sync::mpsc::channel;
/* FP:tests.rs-0006 */ use std::{thread, time};
/* FP:tests.rs-0007 */ 
/* FP:tests.rs-0008 */ use crate::ThreadPoolBuilder;
/* FP:tests.rs-0009 */ 
/* FP:tests.rs-0010 */ #[test]
/* FP:tests.rs-0011 */ fn broadcast_global() {
/* FP:tests.rs-0012 */     let v = crate::broadcast(|ctx| ctx.index());
/* FP:tests.rs-0013 */     assert!(v.into_iter().eq(0..crate::current_num_threads()));
/* FP:tests.rs-0014 */ }
/* FP:tests.rs-0015 */ 
/* FP:tests.rs-0016 */ #[test]
/* FP:tests.rs-0017 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0018 */ fn spawn_broadcast_global() {
/* FP:tests.rs-0019 */     let (tx, rx) = channel();
/* FP:tests.rs-0020 */     crate::spawn_broadcast(move |ctx| tx.send(ctx.index()).unwrap());
/* FP:tests.rs-0021 */ 
/* FP:tests.rs-0022 */     let mut v: Vec<_> = rx.into_iter().collect();
/* FP:tests.rs-0023 */     v.sort_unstable();
/* FP:tests.rs-0024 */     assert!(v.into_iter().eq(0..crate::current_num_threads()));
/* FP:tests.rs-0025 */ }
/* FP:tests.rs-0026 */ 
/* FP:tests.rs-0027 */ #[test]
/* FP:tests.rs-0028 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0029 */ fn broadcast_pool() {
/* FP:tests.rs-0030 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0031 */     let v = pool.broadcast(|ctx| ctx.index());
/* FP:tests.rs-0032 */     assert!(v.into_iter().eq(0..7));
/* FP:tests.rs-0033 */ }
/* FP:tests.rs-0034 */ 
/* FP:tests.rs-0035 */ #[test]
/* FP:tests.rs-0036 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0037 */ fn spawn_broadcast_pool() {
/* FP:tests.rs-0038 */     let (tx, rx) = channel();
/* FP:tests.rs-0039 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0040 */     pool.spawn_broadcast(move |ctx| tx.send(ctx.index()).unwrap());
/* FP:tests.rs-0041 */ 
/* FP:tests.rs-0042 */     let mut v: Vec<_> = rx.into_iter().collect();
/* FP:tests.rs-0043 */     v.sort_unstable();
/* FP:tests.rs-0044 */     assert!(v.into_iter().eq(0..7));
/* FP:tests.rs-0045 */ }
/* FP:tests.rs-0046 */ 
/* FP:tests.rs-0047 */ #[test]
/* FP:tests.rs-0048 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0049 */ fn broadcast_self() {
/* FP:tests.rs-0050 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0051 */     let v = pool.install(|| crate::broadcast(|ctx| ctx.index()));
/* FP:tests.rs-0052 */     assert!(v.into_iter().eq(0..7));
/* FP:tests.rs-0053 */ }
/* FP:tests.rs-0054 */ 
/* FP:tests.rs-0055 */ #[test]
/* FP:tests.rs-0056 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0057 */ fn spawn_broadcast_self() {
/* FP:tests.rs-0058 */     let (tx, rx) = channel();
/* FP:tests.rs-0059 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0060 */     pool.spawn(|| crate::spawn_broadcast(move |ctx| tx.send(ctx.index()).unwrap()));
/* FP:tests.rs-0061 */ 
/* FP:tests.rs-0062 */     let mut v: Vec<_> = rx.into_iter().collect();
/* FP:tests.rs-0063 */     v.sort_unstable();
/* FP:tests.rs-0064 */     assert!(v.into_iter().eq(0..7));
/* FP:tests.rs-0065 */ }
/* FP:tests.rs-0066 */ 
/* FP:tests.rs-0067 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0068 */ #[test]
/* FP:tests.rs-0069 */ #[ignore]
/* FP:tests.rs-0070 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0071 */ fn broadcast_mutual() {
/* FP:tests.rs-0072 */     let count = AtomicUsize::new(0);
/* FP:tests.rs-0073 */     let pool1 = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
/* FP:tests.rs-0074 */     let pool2 = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0075 */     pool1.install(|| {
/* FP:tests.rs-0076 */         pool2.broadcast(|_| {
/* FP:tests.rs-0077 */             pool1.broadcast(|_| {
/* FP:tests.rs-0078 */                 count.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0079 */             })
/* FP:tests.rs-0080 */         })
/* FP:tests.rs-0081 */     });
/* FP:tests.rs-0082 */     assert_eq!(count.into_inner(), 3 * 7);
/* FP:tests.rs-0083 */ }
/* FP:tests.rs-0084 */ 
/* FP:tests.rs-0085 */ #[test]
/* FP:tests.rs-0086 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0087 */ fn spawn_broadcast_mutual() {
/* FP:tests.rs-0088 */     let (tx, rx) = channel();
/* FP:tests.rs-0089 */     let pool1 = Arc::new(ThreadPoolBuilder::new().num_threads(3).build().unwrap());
/* FP:tests.rs-0090 */     let pool2 = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0091 */     pool1.spawn({
/* FP:tests.rs-0092 */         let pool1 = Arc::clone(&pool1);
/* FP:tests.rs-0093 */         move || {
/* FP:tests.rs-0094 */             pool2.spawn_broadcast(move |_| {
/* FP:tests.rs-0095 */                 let tx = tx.clone();
/* FP:tests.rs-0096 */                 pool1.spawn_broadcast(move |_| tx.send(()).unwrap())
/* FP:tests.rs-0097 */             })
/* FP:tests.rs-0098 */         }
/* FP:tests.rs-0099 */     });
/* FP:tests.rs-0100 */     assert_eq!(rx.into_iter().count(), 3 * 7);
/* FP:tests.rs-0101 */ }
/* FP:tests.rs-0102 */ 
/* FP:tests.rs-0103 */ // FIXME: We should fix or remove this ignored test.
/* FP:tests.rs-0104 */ #[test]
/* FP:tests.rs-0105 */ #[ignore]
/* FP:tests.rs-0106 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0107 */ fn broadcast_mutual_sleepy() {
/* FP:tests.rs-0108 */     let count = AtomicUsize::new(0);
/* FP:tests.rs-0109 */     let pool1 = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
/* FP:tests.rs-0110 */     let pool2 = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0111 */     pool1.install(|| {
/* FP:tests.rs-0112 */         thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0113 */         pool2.broadcast(|_| {
/* FP:tests.rs-0114 */             thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0115 */             pool1.broadcast(|_| {
/* FP:tests.rs-0116 */                 thread::sleep(time::Duration::from_millis(100));
/* FP:tests.rs-0117 */                 count.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0118 */             })
/* FP:tests.rs-0119 */         })
/* FP:tests.rs-0120 */     });
/* FP:tests.rs-0121 */     assert_eq!(count.into_inner(), 3 * 7);
/* FP:tests.rs-0122 */ }
/* FP:tests.rs-0123 */ 
/* FP:tests.rs-0124 */ #[test]
/* FP:tests.rs-0125 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0126 */ fn spawn_broadcast_mutual_sleepy() {
/* FP:tests.rs-0127 */     let (tx, rx) = channel();
/* FP:tests.rs-0128 */     let pool1 = Arc::new(ThreadPoolBuilder::new().num_threads(3).build().unwrap());
/* FP:tests.rs-0129 */     let pool2 = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0130 */     pool1.spawn({
/* FP:tests.rs-0131 */         let pool1 = Arc::clone(&pool1);
/* FP:tests.rs-0132 */         move || {
/* FP:tests.rs-0133 */             thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0134 */             pool2.spawn_broadcast(move |_| {
/* FP:tests.rs-0135 */                 let tx = tx.clone();
/* FP:tests.rs-0136 */                 thread::sleep(time::Duration::from_secs(1));
/* FP:tests.rs-0137 */                 pool1.spawn_broadcast(move |_| {
/* FP:tests.rs-0138 */                     thread::sleep(time::Duration::from_millis(100));
/* FP:tests.rs-0139 */                     tx.send(()).unwrap();
/* FP:tests.rs-0140 */                 })
/* FP:tests.rs-0141 */             })
/* FP:tests.rs-0142 */         }
/* FP:tests.rs-0143 */     });
/* FP:tests.rs-0144 */     assert_eq!(rx.into_iter().count(), 3 * 7);
/* FP:tests.rs-0145 */ }
/* FP:tests.rs-0146 */ 
/* FP:tests.rs-0147 */ #[test]
/* FP:tests.rs-0148 */ #[cfg_attr(not(panic = "unwind"), ignore)]
/* FP:tests.rs-0149 */ fn broadcast_panic_one() {
/* FP:tests.rs-0150 */     let count = AtomicUsize::new(0);
/* FP:tests.rs-0151 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0152 */     let result = crate::unwind::halt_unwinding(|| {
/* FP:tests.rs-0153 */         pool.broadcast(|ctx| {
/* FP:tests.rs-0154 */             count.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0155 */             if ctx.index() == 3 {
/* FP:tests.rs-0156 */                 panic!("Hello, world!");
/* FP:tests.rs-0157 */             }
/* FP:tests.rs-0158 */         })
/* FP:tests.rs-0159 */     });
/* FP:tests.rs-0160 */     assert_eq!(count.into_inner(), 7);
/* FP:tests.rs-0161 */     assert!(result.is_err(), "broadcast panic should propagate!");
/* FP:tests.rs-0162 */ }
/* FP:tests.rs-0163 */ 
/* FP:tests.rs-0164 */ #[test]
/* FP:tests.rs-0165 */ #[cfg_attr(not(panic = "unwind"), ignore)]
/* FP:tests.rs-0166 */ fn spawn_broadcast_panic_one() {
/* FP:tests.rs-0167 */     let (tx, rx) = channel();
/* FP:tests.rs-0168 */     let (panic_tx, panic_rx) = channel();
/* FP:tests.rs-0169 */     let pool = ThreadPoolBuilder::new()
/* FP:tests.rs-0170 */         .num_threads(7)
/* FP:tests.rs-0171 */         .panic_handler(move |e| panic_tx.send(e).unwrap())
/* FP:tests.rs-0172 */         .build()
/* FP:tests.rs-0173 */         .unwrap();
/* FP:tests.rs-0174 */     pool.spawn_broadcast(move |ctx| {
/* FP:tests.rs-0175 */         tx.send(()).unwrap();
/* FP:tests.rs-0176 */         if ctx.index() == 3 {
/* FP:tests.rs-0177 */             panic!("Hello, world!");
/* FP:tests.rs-0178 */         }
/* FP:tests.rs-0179 */     });
/* FP:tests.rs-0180 */     drop(pool); // including panic_tx
/* FP:tests.rs-0181 */     assert_eq!(rx.into_iter().count(), 7);
/* FP:tests.rs-0182 */     assert_eq!(panic_rx.into_iter().count(), 1);
/* FP:tests.rs-0183 */ }
/* FP:tests.rs-0184 */ 
/* FP:tests.rs-0185 */ #[test]
/* FP:tests.rs-0186 */ #[cfg_attr(not(panic = "unwind"), ignore)]
/* FP:tests.rs-0187 */ fn broadcast_panic_many() {
/* FP:tests.rs-0188 */     let count = AtomicUsize::new(0);
/* FP:tests.rs-0189 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0190 */     let result = crate::unwind::halt_unwinding(|| {
/* FP:tests.rs-0191 */         pool.broadcast(|ctx| {
/* FP:tests.rs-0192 */             count.fetch_add(1, Ordering::Relaxed);
/* FP:tests.rs-0193 */             if ctx.index() % 2 == 0 {
/* FP:tests.rs-0194 */                 panic!("Hello, world!");
/* FP:tests.rs-0195 */             }
/* FP:tests.rs-0196 */         })
/* FP:tests.rs-0197 */     });
/* FP:tests.rs-0198 */     assert_eq!(count.into_inner(), 7);
/* FP:tests.rs-0199 */     assert!(result.is_err(), "broadcast panic should propagate!");
/* FP:tests.rs-0200 */ }
/* FP:tests.rs-0201 */ 
/* FP:tests.rs-0202 */ #[test]
/* FP:tests.rs-0203 */ #[cfg_attr(not(panic = "unwind"), ignore)]
/* FP:tests.rs-0204 */ fn spawn_broadcast_panic_many() {
/* FP:tests.rs-0205 */     let (tx, rx) = channel();
/* FP:tests.rs-0206 */     let (panic_tx, panic_rx) = channel();
/* FP:tests.rs-0207 */     let pool = ThreadPoolBuilder::new()
/* FP:tests.rs-0208 */         .num_threads(7)
/* FP:tests.rs-0209 */         .panic_handler(move |e| panic_tx.send(e).unwrap())
/* FP:tests.rs-0210 */         .build()
/* FP:tests.rs-0211 */         .unwrap();
/* FP:tests.rs-0212 */     pool.spawn_broadcast(move |ctx| {
/* FP:tests.rs-0213 */         tx.send(()).unwrap();
/* FP:tests.rs-0214 */         if ctx.index() % 2 == 0 {
/* FP:tests.rs-0215 */             panic!("Hello, world!");
/* FP:tests.rs-0216 */         }
/* FP:tests.rs-0217 */     });
/* FP:tests.rs-0218 */     drop(pool); // including panic_tx
/* FP:tests.rs-0219 */     assert_eq!(rx.into_iter().count(), 7);
/* FP:tests.rs-0220 */     assert_eq!(panic_rx.into_iter().count(), 4);
/* FP:tests.rs-0221 */ }
/* FP:tests.rs-0222 */ 
/* FP:tests.rs-0223 */ #[test]
/* FP:tests.rs-0224 */ #[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
/* FP:tests.rs-0225 */ fn broadcast_sleep_race() {
/* FP:tests.rs-0226 */     let test_duration = time::Duration::from_secs(1);
/* FP:tests.rs-0227 */     let pool = ThreadPoolBuilder::new().num_threads(7).build().unwrap();
/* FP:tests.rs-0228 */     let start = time::Instant::now();
/* FP:tests.rs-0229 */     while start.elapsed() < test_duration {
/* FP:tests.rs-0230 */         pool.broadcast(|ctx| {
/* FP:tests.rs-0231 */             // A slight spread of sleep duration increases the chance that one
/* FP:tests.rs-0232 */             // of the threads will race in the pool's idle sleep afterward.
/* FP:tests.rs-0233 */             thread::sleep(time::Duration::from_micros(ctx.index() as u64));
/* FP:tests.rs-0234 */         });
/* FP:tests.rs-0235 */     }
/* FP:tests.rs-0236 */ }
/* FP:tests.rs-0237 */ 
/* FP:tests.rs-0238 */ #[test]
/* FP:tests.rs-0239 */ fn broadcast_after_spawn_broadcast() {
/* FP:tests.rs-0240 */     let (tx, rx) = channel();
/* FP:tests.rs-0241 */ 
/* FP:tests.rs-0242 */     // Queue a non-blocking spawn_broadcast.
/* FP:tests.rs-0243 */     crate::spawn_broadcast(move |ctx| tx.send(ctx.index()).unwrap());
/* FP:tests.rs-0244 */ 
/* FP:tests.rs-0245 */     // This blocking broadcast runs after all prior broadcasts.
/* FP:tests.rs-0246 */     crate::broadcast(|_| {});
/* FP:tests.rs-0247 */ 
/* FP:tests.rs-0248 */     // The spawn_broadcast **must** have run by now on all threads.
/* FP:tests.rs-0249 */     let mut v: Vec<_> = rx.try_iter().collect();
/* FP:tests.rs-0250 */     v.sort_unstable();
/* FP:tests.rs-0251 */     assert!(v.into_iter().eq(0..crate::current_num_threads()));
/* FP:tests.rs-0252 */ }
/* FP:tests.rs-0253 */ 
/* FP:tests.rs-0254 */ #[test]
/* FP:tests.rs-0255 */ fn broadcast_after_spawn() {
/* FP:tests.rs-0256 */     let (tx, rx) = channel();
/* FP:tests.rs-0257 */ 
/* FP:tests.rs-0258 */     // Queue a regular spawn on a thread-local deque.
/* FP:tests.rs-0259 */     crate::registry::in_worker(move |_, _| {
/* FP:tests.rs-0260 */         crate::spawn(move || tx.send(22).unwrap());
/* FP:tests.rs-0261 */     });
/* FP:tests.rs-0262 */ 
/* FP:tests.rs-0263 */     // Broadcast runs after the local deque is empty.
/* FP:tests.rs-0264 */     crate::broadcast(|_| {});
/* FP:tests.rs-0265 */ 
/* FP:tests.rs-0266 */     // The spawn **must** have run by now.
/* FP:tests.rs-0267 */     assert_eq!(22, rx.try_recv().unwrap());
/* FP:tests.rs-0268 */ }