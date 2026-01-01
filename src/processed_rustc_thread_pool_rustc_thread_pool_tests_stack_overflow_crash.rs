/* FP:stack_overflow_crash.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_USE_0001
/* FP:stack_overflow_crash.rs-0002 */ # [allow (unused_crate_dependencies)] use std :: env ;
/* FP:stack_overflow_crash.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_USE_0002
/* FP:stack_overflow_crash.rs-0004 */ # [cfg (target_os = "linux")] use std :: os :: unix :: process :: ExitStatusExt ;
/* FP:stack_overflow_crash.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_USE_0003
/* FP:stack_overflow_crash.rs-0006 */ use std :: process :: { Command , ExitStatus , Stdio } ;
/* FP:stack_overflow_crash.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_USE_0004
/* FP:stack_overflow_crash.rs-0008 */ use crate :: rustc_thread_pool :: ThreadPoolBuilder ;
/* FP:stack_overflow_crash.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0005
/* FP:stack_overflow_crash.rs-0010 */ fn force_stack_overflow (depth : u32) { let mut buffer = [0u8 ; 1024 * 1024] ; # [allow (clippy :: incompatible_msrv)] std :: hint :: black_box (& mut buffer) ; if depth > 0 { force_stack_overflow (depth - 1) ; } }
/* FP:stack_overflow_crash.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0006
/* FP:stack_overflow_crash.rs-0012 */ # [cfg (unix)] fn disable_core () { unsafe { libc :: setrlimit (libc :: RLIMIT_CORE , & libc :: rlimit { rlim_cur : 0 , rlim_max : 0 }) ; } }
/* FP:stack_overflow_crash.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0007
/* FP:stack_overflow_crash.rs-0014 */ # [cfg (unix)] fn overflow_code () -> Option < i32 > { None }
/* FP:stack_overflow_crash.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0008
/* FP:stack_overflow_crash.rs-0016 */ # [cfg (windows)] fn overflow_code () -> Option < i32 > { use std :: os :: windows :: process :: ExitStatusExt ; ExitStatus :: from_raw (0xc00000fd) . code () }
/* FP:stack_overflow_crash.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0009
/* FP:stack_overflow_crash.rs-0018 */ # [test] # [cfg_attr (not (any (unix)) , ignore)] fn stack_overflow_crash () { let status = run_ignored ("run_with_small_stack") ; assert ! (! status . success ()) ; # [cfg (any (unix , windows))] assert_eq ! (status . code () , overflow_code ()) ; # [cfg (target_os = "linux")] assert ! (matches ! (status . signal () , Some (libc :: SIGABRT | libc :: SIGSEGV))) ; let status = run_ignored ("run_with_large_stack") ; assert_eq ! (status . code () , Some (0)) ; # [cfg (target_os = "linux")] assert_eq ! (status . signal () , None) ; }
/* FP:stack_overflow_crash.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0010
/* FP:stack_overflow_crash.rs-0020 */ fn run_ignored (test : & str) -> ExitStatus { Command :: new (env :: current_exe () . unwrap ()) . arg ("--ignored") . arg ("--exact") . arg (test) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () . unwrap () }
/* FP:stack_overflow_crash.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0011
/* FP:stack_overflow_crash.rs-0022 */ # [test] # [ignore] fn run_with_small_stack () { run_with_stack (8) ; }
/* FP:stack_overflow_crash.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0012
/* FP:stack_overflow_crash.rs-0024 */ # [test] # [ignore] fn run_with_large_stack () { run_with_stack (48) ; }
/* FP:stack_overflow_crash.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_stack_overflow_crash_FN_0013
/* FP:stack_overflow_crash.rs-0026 */ fn run_with_stack (stack_size_in_mb : usize) { let pool = ThreadPoolBuilder :: new () . stack_size (stack_size_in_mb * 1024 * 1024) . build () . unwrap () ; pool . install (| | { # [cfg (unix)] disable_core () ; force_stack_overflow (32) ; }) ; }