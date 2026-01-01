/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_FN_0002
/* FP:tests.rs-0004 */ # [test] fn short () { const TEXT : & str = "abcd" ; let reference = ffi :: CString :: new (TEXT . to_string ()) . unwrap () ; let scs = SmallCStr :: new (TEXT) ; assert_eq ! (scs . len_with_nul () , TEXT . len () + 1) ; assert_eq ! (scs . as_c_str () , reference . as_c_str ()) ; assert ! (! scs . spilled ()) ; }
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn empty () { const TEXT : & str = "" ; let reference = ffi :: CString :: new (TEXT . to_string ()) . unwrap () ; let scs = SmallCStr :: new (TEXT) ; assert_eq ! (scs . len_with_nul () , TEXT . len () + 1) ; assert_eq ! (scs . as_c_str () , reference . as_c_str ()) ; assert ! (! scs . spilled ()) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn long () { const TEXT : & str = "01234567890123456789012345678901234567890123456789\
/* FP:tests.rs-0009 */                         01234567890123456789012345678901234567890123456789\
/* FP:tests.rs-0010 */                         01234567890123456789012345678901234567890123456789" ; let reference = ffi :: CString :: new (TEXT . to_string ()) . unwrap () ; let scs = SmallCStr :: new (TEXT) ; assert_eq ! (scs . len_with_nul () , TEXT . len () + 1) ; assert_eq ! (scs . as_c_str () , reference . as_c_str ()) ; assert ! (scs . spilled ()) ; }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_FN_0005
/* FP:tests.rs-0012 */ # [test] # [should_panic] fn internal_nul () { let _ = SmallCStr :: new ("abcd\0def") ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_small_c_str_tests_FN_0006
/* FP:tests.rs-0014 */ # [test] fn from_cstr () { let c = c"foo" ; let s : SmallCStr = c . into () ; assert_eq ! (s . len_with_nul () , 4) ; assert_eq ! (s . as_c_str () , c"foo") ; }