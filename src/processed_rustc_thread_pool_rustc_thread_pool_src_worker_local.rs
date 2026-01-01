/* FP:worker_local.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_USE_0001
/* FP:worker_local.rs-0002 */ use std :: fmt ;
/* FP:worker_local.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_USE_0002
/* FP:worker_local.rs-0004 */ use std :: ops :: Deref ;
/* FP:worker_local.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_USE_0003
/* FP:worker_local.rs-0006 */ use std :: sync :: Arc ;
/* FP:worker_local.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_USE_0004
/* FP:worker_local.rs-0008 */ use crate :: registry :: { Registry , WorkerThread } ;
/* FP:worker_local.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_STRUCT_0005
/* FP:worker_local.rs-0010 */ # [repr (align (64))] # [derive (Debug)] struct CacheAligned < T > (T) ;
/* FP:worker_local.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_STRUCT_0006
/* FP:worker_local.rs-0012 */ # [doc = " Holds worker-locals values for each thread in a thread pool."] # [doc = " You can only access the worker local value through the Deref impl"] # [doc = " on the thread pool it was constructed on. It will panic otherwise"] pub struct WorkerLocal < T > { locals : Vec < CacheAligned < T > > , registry : Arc < Registry > , }
/* FP:worker_local.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_IMPL_0007
/* FP:worker_local.rs-0014 */ # [doc = " We prevent concurrent access to the underlying value in the"] # [doc = " Deref impl, thus any values safe to send across threads can"] # [doc = " be used with WorkerLocal."] unsafe impl < T : Send > Sync for WorkerLocal < T > { }
/* FP:worker_local.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_IMPL_0008
/* FP:worker_local.rs-0016 */ impl < T > WorkerLocal < T > { # [doc = " Creates a new worker local where the `initial` closure computes the"] # [doc = " value this worker local should take for each thread in the thread pool."] # [inline] pub fn new < F : FnMut (usize) -> T > (mut initial : F) -> WorkerLocal < T > { let registry = Registry :: current () ; WorkerLocal { locals : (0 .. registry . num_threads ()) . map (| i | CacheAligned (initial (i))) . collect () , registry , } } # [doc = " Returns the worker-local value for each thread"] # [inline] pub fn into_inner (self) -> Vec < T > { self . locals . into_iter () . map (| c | c . 0) . collect () } fn current (& self) -> & T { unsafe { let worker_thread = WorkerThread :: current () ; if worker_thread . is_null () || & * (* worker_thread) . registry as * const _ != & * self . registry as * const _ { panic ! ("WorkerLocal can only be used on the thread pool it was created on") } & self . locals [(* worker_thread) . index] . 0 } } }
/* FP:worker_local.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_IMPL_0009
/* FP:worker_local.rs-0018 */ impl < T > WorkerLocal < Vec < T > > { # [doc = " Joins the elements of all the worker locals into one Vec"] pub fn join (self) -> Vec < T > { self . into_inner () . into_iter () . flat_map (| v | v) . collect () } }
/* FP:worker_local.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_IMPL_0010
/* FP:worker_local.rs-0020 */ impl < T : fmt :: Debug > fmt :: Debug for WorkerLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WorkerLocal") . field ("registry" , & self . registry . id ()) . finish () } }
/* FP:worker_local.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_worker_local_IMPL_0011
/* FP:worker_local.rs-0022 */ impl < T > Deref for WorkerLocal < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { self . current () } }