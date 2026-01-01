/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_symbol_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_symbol_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: create_default_session_globals_then ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_symbol_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn interner_tests () { let i = Interner :: prefill (& [] , & []) ; assert_eq ! (i . intern_str ("dog") , Symbol :: new (0)) ; assert_eq ! (i . intern_byte_str (b"dog") , ByteSymbol :: new (0)) ; assert_eq ! (i . intern_byte_str (b"cat") , ByteSymbol :: new (1)) ; assert_eq ! (i . intern_str ("cat") , Symbol :: new (1)) ; assert_eq ! (i . intern_str ("dog") , Symbol :: new (0)) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_symbol_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn without_first_quote_test () { create_default_session_globals_then (| | { let i = Ident :: from_str ("'break") ; assert_eq ! (i . without_first_quote () . name , kw :: Break) ; }) ; }