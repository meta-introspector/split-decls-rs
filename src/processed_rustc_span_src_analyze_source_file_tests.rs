/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0002
/* FP:tests.rs-0004 */ macro_rules ! test { (case : $ test_name : ident , text : $ text : expr , lines : $ lines : expr , multi_byte_chars : $ multi_byte_chars : expr ,) => { # [test] fn $ test_name () { let (lines , multi_byte_chars) = analyze_source_file ($ text) ; let expected_lines : Vec < RelativeBytePos > = $ lines . into_iter () . map (RelativeBytePos) . collect () ; assert_eq ! (lines , expected_lines) ; let expected_mbcs : Vec < MultiByteChar > = $ multi_byte_chars . into_iter () . map (| (pos , bytes) | MultiByteChar { pos : RelativeBytePos (pos) , bytes }) . collect () ; assert_eq ! (multi_byte_chars , expected_mbcs) ; } } ; }
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0003
/* FP:tests.rs-0006 */ test ! (case : empty_text , text : "" , lines : vec ! [] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0004
/* FP:tests.rs-0008 */ test ! (case : newlines_short , text : "a\nc" , lines : vec ! [0 , 2] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0005
/* FP:tests.rs-0010 */ test ! (case : newlines_long , text : "012345678\nabcdef012345678\na" , lines : vec ! [0 , 10 , 26] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0006
/* FP:tests.rs-0012 */ test ! (case : newline_and_multi_byte_char_in_same_chunk , text : "01234β789\nbcdef0123456789abcdef" , lines : vec ! [0 , 11] , multi_byte_chars : vec ! [(5 , 2)] ,) ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0007
/* FP:tests.rs-0014 */ test ! (case : newline_and_control_char_in_same_chunk , text : "01234\u{07}6789\nbcdef0123456789abcdef" , lines : vec ! [0 , 11] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0008
/* FP:tests.rs-0016 */ test ! (case : multi_byte_char_short , text : "aβc" , lines : vec ! [0] , multi_byte_chars : vec ! [(1 , 2)] ,) ;
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0009
/* FP:tests.rs-0018 */ test ! (case : multi_byte_char_long , text : "0123456789abcΔf012345β" , lines : vec ! [0] , multi_byte_chars : vec ! [(13 , 2) , (22 , 2)] ,) ;
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0010
/* FP:tests.rs-0020 */ test ! (case : multi_byte_char_across_chunk_boundary , text : "0123456789abcdeΔ123456789abcdef01234" , lines : vec ! [0] , multi_byte_chars : vec ! [(15 , 2)] ,) ;
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0011
/* FP:tests.rs-0022 */ test ! (case : multi_byte_char_across_chunk_boundary_tail , text : "0123456789abcdeΔ...." , lines : vec ! [0] , multi_byte_chars : vec ! [(15 , 2)] ,) ;
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0012
/* FP:tests.rs-0024 */ test ! (case : non_narrow_short , text : "0\t2" , lines : vec ! [0] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0013
/* FP:tests.rs-0026 */ test ! (case : non_narrow_long , text : "01\t3456789abcdef01234567\u{07}9" , lines : vec ! [0] , multi_byte_chars : vec ! [] ,) ;
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_span_src_analyze_source_file_tests_MACRO_0014
/* FP:tests.rs-0028 */ test ! (case : output_offset_all , text : "01\t345\n789abcΔf01234567\u{07}9\nbcΔf" , lines : vec ! [0 , 7 , 27] , multi_byte_chars : vec ! [(13 , 2) , (29 , 2)] ,) ;