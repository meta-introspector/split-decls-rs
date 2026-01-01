/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: str ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_complete :: BytePos ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: rustc_complete :: source_map :: FilePathMapping ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0004
/* FP:tests.rs-0008 */ use serde :: Deserialize ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0005
/* FP:tests.rs-0010 */ use super :: * ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_USE_0006
/* FP:tests.rs-0012 */ use crate :: DiagCtxt ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_STRUCT_0007
/* FP:tests.rs-0014 */ # [derive (Deserialize , Debug , PartialEq , Eq)] struct TestData { spans : Vec < SpanTestData > , }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_STRUCT_0008
/* FP:tests.rs-0016 */ # [derive (Deserialize , Debug , PartialEq , Eq)] struct SpanTestData { pub byte_start : u32 , pub byte_end : u32 , pub line_start : u32 , pub column_start : u32 , pub line_end : u32 , pub column_end : u32 , }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_STRUCT_0009
/* FP:tests.rs-0018 */ struct Shared < T > { data : Arc < Mutex < T > > , }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_IMPL_0010
/* FP:tests.rs-0020 */ impl < T : Write > Write for Shared < T > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . data . lock () . unwrap () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . data . lock () . unwrap () . flush () } }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0011
/* FP:tests.rs-0022 */ # [doc = " Test the span yields correct positions in JSON."] fn test_positions (code : & str , span : (u32 , u32) , expected_output : SpanTestData) { crate :: rustc_span :: create_default_session_globals_then (| | { let sm = Arc :: new (SourceMap :: new (FilePathMapping :: empty ())) ; sm . new_source_file (Path :: new ("test.rs") . to_owned () . into () , code . to_owned ()) ; let translator = Translator :: with_fallback_bundle (vec ! [crate :: DEFAULT_LOCALE_RESOURCE] , false) ; let output = Arc :: new (Mutex :: new (Vec :: new ())) ; let je = JsonEmitter :: new (Box :: new (Shared { data : output . clone () }) , Some (sm) , translator , true , HumanReadableErrorType :: Short , ColorConfig :: Never ,) ; let span = Span :: with_root_ctxt (BytePos (span . 0) , BytePos (span . 1)) ; DiagCtxt :: new (Box :: new (je)) . handle () . span_err (span , "foo") ; let bytes = output . lock () . unwrap () ; let actual_output = str :: from_utf8 (& bytes) . unwrap () ; let actual_output : TestData = serde_json :: from_str (actual_output) . unwrap () ; let spans = actual_output . spans ; assert_eq ! (spans . len () , 1) ; assert_eq ! (expected_output , spans [0]) }) }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] fn empty () { test_positions (" " , (0 , 1) , SpanTestData { byte_start : 0 , byte_end : 1 , line_start : 1 , column_start : 1 , line_end : 1 , column_end : 2 , } ,) }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn bom () { test_positions ("\u{feff} " , (0 , 1) , SpanTestData { byte_start : 3 , byte_end : 4 , line_start : 1 , column_start : 1 , line_end : 1 , column_end : 2 , } ,) }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0014
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0015
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0016
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0017
/* FP:tests.rs-0034 */ # [test] fn span_before_crlf () { test_positions ("foo\r\nbar" , (2 , 3) , SpanTestData { byte_start : 2 , byte_end : 3 , line_start : 1 , column_start : 3 , line_end : 1 , column_end : 4 , } ,) }
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0018
/* FP:tests.rs-0036 */ # [test] fn span_on_crlf () { test_positions ("foo\r\nbar" , (3 , 4) , SpanTestData { byte_start : 3 , byte_end : 5 , line_start : 1 , column_start : 4 , line_end : 2 , column_end : 1 , } ,) }
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_json_tests_FN_0019
/* FP:tests.rs-0038 */ # [test] fn span_after_crlf () { test_positions ("foo\r\nbar" , (4 , 5) , SpanTestData { byte_start : 5 , byte_end : 6 , line_start : 2 , column_start : 1 , line_end : 2 , column_end : 2 , } ,) }