/* FP:simple_panic.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_simple_panic_USE_0001
/* FP:simple_panic.rs-0002 */ # [allow (unused_crate_dependencies)] use crate :: rustc_thread_pool :: join ;
/* FP:simple_panic.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_simple_panic_FN_0002
/* FP:simple_panic.rs-0004 */ # [test] # [should_panic (expected = "should panic")] fn simple_panic () { join (| | { } , | | panic ! ("should panic")) ; }