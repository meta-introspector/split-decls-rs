/* FP:unwind.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_USE_0001
/* FP:unwind.rs-0002 */ use std :: any :: Any ;
/* FP:unwind.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_USE_0002
/* FP:unwind.rs-0004 */ use std :: panic :: { self , AssertUnwindSafe } ;
/* FP:unwind.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_USE_0003
/* FP:unwind.rs-0006 */ use std :: thread ;
/* FP:unwind.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_FN_0004
/* FP:unwind.rs-0008 */ # [doc = " Executes `f` and captures any panic, translating that panic into a"] # [doc = " `Err` result. The assumption is that any panic will be propagated"] # [doc = " later with `resume_unwinding`, and hence `f` can be treated as"] # [doc = " exception safe."] pub (super) fn halt_unwinding < F , R > (func : F) -> thread :: Result < R > where F : FnOnce () -> R , { panic :: catch_unwind (AssertUnwindSafe (func)) }
/* FP:unwind.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_FN_0005
/* FP:unwind.rs-0010 */ pub (super) fn resume_unwinding (payload : Box < dyn Any + Send >) -> ! { panic :: resume_unwind (payload) }
/* FP:unwind.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_STRUCT_0006
/* FP:unwind.rs-0012 */ pub (super) struct AbortIfPanic ;
/* FP:unwind.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_unwind_IMPL_0007
/* FP:unwind.rs-0014 */ impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }