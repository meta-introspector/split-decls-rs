/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_USE_0001
/* FP:tests.rs-0002 */ # [allow (rustc :: symbol_intern_string_literal)] use crate :: rustc_complete :: token :: { self , IdentIsRaw } ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_complete :: tokenstream :: { TokenStream , TokenTree } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: rustc_complete :: { BytePos , Span , Symbol , create_default_session_globals_then } ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: parser :: tests :: string_to_stream ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0005
/* FP:tests.rs-0010 */ fn string_to_ts (string : & str) -> TokenStream { string_to_stream (string . to_owned ()) }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0006
/* FP:tests.rs-0012 */ fn sp (a : u32 , b : u32) -> Span { Span :: with_root_ctxt (BytePos (a) , BytePos (b)) }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0007
/* FP:tests.rs-0014 */ fn cmp_token_stream (a : & TokenStream , b : & TokenStream) -> bool { a . len () == b . len () && a . iter () . zip (b . iter ()) . all (| (x , y) | x . eq_unspanned (y)) }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn test_concat () { create_default_session_globals_then (| | { let test_res = string_to_ts ("foo::bar::baz") ; let test_fst = string_to_ts ("foo::bar") ; let test_snd = string_to_ts ("::baz") ; let mut eq_res = TokenStream :: default () ; eq_res . push_stream (test_fst) ; eq_res . push_stream (test_snd) ; assert_eq ! (test_res . iter () . count () , 5) ; assert_eq ! (eq_res . iter () . count () , 5) ; assert_eq ! (cmp_token_stream (& test_res , & eq_res) , true) ; }) }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0009
/* FP:tests.rs-0018 */ # [test] fn test_to_from_bijection () { create_default_session_globals_then (| | { let test_start = string_to_ts ("foo::bar(baz)") ; let test_end = test_start . iter () . cloned () . collect () ; assert_eq ! (test_start , test_end) }) }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0010
/* FP:tests.rs-0020 */ # [test] fn test_eq_0 () { create_default_session_globals_then (| | { let test_res = string_to_ts ("foo") ; let test_eqs = string_to_ts ("foo") ; assert_eq ! (test_res , test_eqs) }) }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0011
/* FP:tests.rs-0022 */ # [test] fn test_eq_1 () { create_default_session_globals_then (| | { let test_res = string_to_ts ("::bar::baz") ; let test_eqs = string_to_ts ("::bar::baz") ; assert_eq ! (test_res , test_eqs) }) }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] fn test_eq_3 () { create_default_session_globals_then (| | { let test_res = string_to_ts ("") ; let test_eqs = string_to_ts ("") ; assert_eq ! (test_res , test_eqs) }) }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn test_diseq_0 () { create_default_session_globals_then (| | { let test_res = string_to_ts ("::bar::baz") ; let test_eqs = string_to_ts ("bar::baz") ; assert_eq ! (test_res == test_eqs , false) }) }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0014
/* FP:tests.rs-0028 */ # [test] fn test_diseq_1 () { create_default_session_globals_then (| | { let test_res = string_to_ts ("(bar,baz)") ; let test_eqs = string_to_ts ("bar,baz") ; assert_eq ! (test_res == test_eqs , false) }) }
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0015
/* FP:tests.rs-0030 */ # [test] fn test_is_empty () { create_default_session_globals_then (| | { let test0 = TokenStream :: default () ; let test1 = TokenStream :: token_alone (token :: Ident (Symbol :: intern ("a") , IdentIsRaw :: No) , sp (0 , 1)) ; let test2 = string_to_ts ("foo(bar::baz)") ; assert_eq ! (test0 . is_empty () , true) ; assert_eq ! (test1 . is_empty () , false) ; assert_eq ! (test2 . is_empty () , false) ; }) }
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tokenstream_tests_FN_0016
/* FP:tests.rs-0032 */ # [test] fn test_dotdotdot () { create_default_session_globals_then (| | { let mut stream = TokenStream :: default () ; stream . push_tree (TokenTree :: token_joint (token :: Dot , sp (0 , 1))) ; stream . push_tree (TokenTree :: token_joint (token :: Dot , sp (1 , 2))) ; stream . push_tree (TokenTree :: token_alone (token :: Dot , sp (2 , 3))) ; assert ! (cmp_token_stream (& stream , & string_to_ts ("..."))) ; assert_eq ! (stream . iter () . count () , 1) ; }) }