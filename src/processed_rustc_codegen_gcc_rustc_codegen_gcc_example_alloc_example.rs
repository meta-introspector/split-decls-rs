/* FP:alloc_example.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_USE_0001
/* FP:alloc_example.rs-0002 */ # [feature (core_intrinsics , alloc_error_handler , lang_items)] # [no_std] # [no_main] # [allow (internal_features)] use alloc :: boxed :: Box ;
/* FP:alloc_example.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_USE_0002
/* FP:alloc_example.rs-0004 */ use alloc_system :: System ;
/* FP:alloc_example.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_STATIC_0003
/* FP:alloc_example.rs-0006 */ # [global_allocator] static ALLOC : System = System ;
/* FP:alloc_example.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_OTHER_0004
/* FP:alloc_example.rs-0008 */ # [link (name = "c")] unsafe extern "C" { fn puts (s : * const u8) -> i32 ; }
/* FP:alloc_example.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_FN_0005
/* FP:alloc_example.rs-0010 */ # [panic_handler] fn panic_handler (_ : & core :: panic :: PanicInfo < '_ >) -> ! { core :: intrinsics :: abort () ; }
/* FP:alloc_example.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_FN_0006
/* FP:alloc_example.rs-0012 */ # [alloc_error_handler] fn alloc_error_handler (_ : alloc :: alloc :: Layout) -> ! { core :: intrinsics :: abort () ; }
/* FP:alloc_example.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_FN_0007
/* FP:alloc_example.rs-0014 */ # [lang = "eh_personality"] fn eh_personality () -> ! { loop { } }
/* FP:alloc_example.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_FN_0008
/* FP:alloc_example.rs-0016 */ # [unsafe (no_mangle)] unsafe extern "C" fn _Unwind_Resume () { core :: intrinsics :: unreachable () ; }
/* FP:alloc_example.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_example_FN_0009
/* FP:alloc_example.rs-0018 */ # [unsafe (no_mangle)] extern "C" fn main (_argc : core :: ffi :: c_int , _argv : * const * const u8) -> core :: ffi :: c_int { let world : Box < & str > = Box :: new ("Hello World!\0") ; unsafe { puts (* world as * const str as * const u8) ; } 0 }