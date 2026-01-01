/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0001
/* FP:tests.rs-0002 */ # [allow (rustc :: symbol_intern_string_literal)] use std :: assert_matches :: assert_matches ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0002
/* FP:tests.rs-0004 */ use std :: io :: prelude :: * ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0003
/* FP:tests.rs-0006 */ use std :: iter :: Peekable ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0004
/* FP:tests.rs-0008 */ use std :: path :: { Path , PathBuf } ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0005
/* FP:tests.rs-0010 */ use std :: sync :: { Arc , Mutex } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0006
/* FP:tests.rs-0012 */ use std :: { io , str } ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0007
/* FP:tests.rs-0014 */ use ast :: token :: IdentIsRaw ;
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0008
/* FP:tests.rs-0016 */ use crate :: rustc_complete :: token :: { self , Delimiter , Token } ;
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0009
/* FP:tests.rs-0018 */ use crate :: rustc_complete :: tokenstream :: { DelimSpacing , DelimSpan , Spacing , TokenStream , TokenTree } ;
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0010
/* FP:tests.rs-0020 */ use crate :: rustc_complete :: { self as ast , PatKind , visit } ;
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0011
/* FP:tests.rs-0022 */ use rustc_ast_pretty :: pprust :: item_to_string ;
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0012
/* FP:tests.rs-0024 */ use crate :: rustc_complete :: emitter :: { HumanEmitter , OutputTheme } ;
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0013
/* FP:tests.rs-0026 */ use crate :: rustc_complete :: translation :: Translator ;
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0014
/* FP:tests.rs-0028 */ use crate :: rustc_complete :: { DiagCtxt , MultiSpan , PResult } ;
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0015
/* FP:tests.rs-0030 */ use crate :: rustc_complete :: parse :: ParseSess ;
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0016
/* FP:tests.rs-0032 */ use crate :: rustc_complete :: source_map :: { FilePathMapping , SourceMap } ;
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0017
/* FP:tests.rs-0034 */ use crate :: rustc_complete :: { BytePos , FileName , Pos , Span , Symbol , create_default_session_globals_then , kw , sym , } ;
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0018
/* FP:tests.rs-0036 */ use termcolor :: WriteColor ;
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0019
/* FP:tests.rs-0038 */ use crate :: lexer :: StripTokens ;
/* FP:tests.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0020
/* FP:tests.rs-0040 */ use crate :: parser :: { ForceCollect , Parser } ;
/* FP:tests.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_USE_0021
/* FP:tests.rs-0042 */ use crate :: { new_parser_from_source_str , source_str_to_stream , unwrap_or_emit_fatal } ;
/* FP:tests.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0022
/* FP:tests.rs-0044 */ fn psess () -> ParseSess { ParseSess :: new (vec ! [crate :: DEFAULT_LOCALE_RESOURCE]) }
/* FP:tests.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0023
/* FP:tests.rs-0046 */ # [doc = " Map string to parser (via tts)."] fn string_to_parser (psess : & ParseSess , source_str : String) -> Parser < '_ > { unwrap_or_emit_fatal (new_parser_from_source_str (psess , PathBuf :: from ("bogofile") . into () , source_str , StripTokens :: Nothing ,)) }
/* FP:tests.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0024
/* FP:tests.rs-0048 */ fn create_test_handler (theme : OutputTheme) -> (DiagCtxt , Arc < SourceMap > , Arc < Mutex < Vec < u8 > > >) { let output = Arc :: new (Mutex :: new (Vec :: new ())) ; let source_map = Arc :: new (SourceMap :: new (FilePathMapping :: empty ())) ; let translator = Translator :: with_fallback_bundle (vec ! [crate :: DEFAULT_LOCALE_RESOURCE] , false) ; let mut emitter = HumanEmitter :: new (Box :: new (Shared { data : output . clone () }) , translator) . sm (Some (source_map . clone ())) . diagnostic_width (Some (140)) ; emitter = emitter . theme (theme) ; let dcx = DiagCtxt :: new (Box :: new (emitter)) ; (dcx , source_map , output) }
/* FP:tests.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0025
/* FP:tests.rs-0050 */ # [doc = " Returns the result of parsing the given string via the given callback."] # [doc = ""] # [doc = " If there are any errors, this will panic."] fn with_error_checking_parse < 'a , T , F > (s : String , psess : & 'a ParseSess , f : F) -> T where F : FnOnce (& mut Parser < 'a >) -> PResult < 'a , T > , { let mut p = string_to_parser (& psess , s) ; let x = f (& mut p) . unwrap () ; p . dcx () . abort_if_errors () ; x }
/* FP:tests.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0026
/* FP:tests.rs-0052 */ # [doc = " Verifies that parsing the given string using the given callback will"] # [doc = " generate an error that contains the given text."] fn with_expected_parse_error < T , F > (source_str : & str , expected_output : & str , f : F) where F : for < 'a > FnOnce (& mut Parser < 'a >) -> PResult < 'a , T > , { let (handler , source_map , output) = create_test_handler (OutputTheme :: Ascii) ; let psess = ParseSess :: with_dcx (handler , source_map) ; let mut p = string_to_parser (& psess , source_str . to_string ()) ; let result = f (& mut p) ; assert ! (result . is_ok ()) ; let bytes = output . lock () . unwrap () ; let actual_output = str :: from_utf8 (& bytes) . unwrap () ; println ! ("expected output:\n------\n{}------" , expected_output) ; println ! ("actual output:\n------\n{}------" , actual_output) ; assert ! (actual_output . contains (expected_output)) }
/* FP:tests.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0027
/* FP:tests.rs-0054 */ # [doc = " Maps a string to tts, using a made-up filename."] pub (crate) fn string_to_stream (source_str : String) -> TokenStream { let psess = psess () ; unwrap_or_emit_fatal (source_str_to_stream (& psess , PathBuf :: from ("bogofile") . into () , source_str , None ,)) }
/* FP:tests.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0028
/* FP:tests.rs-0056 */ # [doc = " Does the given string match the pattern? whitespace in the first string"] # [doc = " may be deleted or replaced with other whitespace to match the pattern."] # [doc = " This function is relatively Unicode-ignorant; fortunately, the careful design"] # [doc = " of UTF-8 mitigates this ignorance. It doesn't do NKF-normalization(?)."] pub (crate) fn matches_codepattern (a : & str , b : & str) -> bool { let mut a_iter = a . chars () . peekable () ; let mut b_iter = b . chars () . peekable () ; loop { let (a , b) = match (a_iter . peek () , b_iter . peek ()) { (None , None) => return true , (None , _) => return false , (Some (& a) , None) => { if rustc_lexer :: is_whitespace (a) { break ; } else { return false ; } } (Some (& a) , Some (& b)) => (a , b) , } ; if rustc_lexer :: is_whitespace (a) && rustc_lexer :: is_whitespace (b) { scan_for_non_ws_or_end (& mut a_iter) ; scan_for_non_ws_or_end (& mut b_iter) ; } else if rustc_lexer :: is_whitespace (a) { scan_for_non_ws_or_end (& mut a_iter) ; } else if a == b { a_iter . next () ; b_iter . next () ; } else { return false ; } } a_iter . all (rustc_lexer :: is_whitespace) }
/* FP:tests.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0029
/* FP:tests.rs-0058 */ # [doc = " Advances the given peekable `Iterator` until it reaches a non-whitespace character."] fn scan_for_non_ws_or_end < I : Iterator < Item = char > > (iter : & mut Peekable < I >) { while iter . peek () . copied () . is_some_and (rustc_lexer :: is_whitespace) { iter . next () ; } }
/* FP:tests.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_STRUCT_0030
/* FP:tests.rs-0060 */ # [doc = " Identifies a position in the text by the n'th occurrence of a string."] struct Position { string : & 'static str , count : usize , }
/* FP:tests.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_STRUCT_0031
/* FP:tests.rs-0062 */ struct SpanLabel { start : Position , end : Position , label : & 'static str , }
/* FP:tests.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_STRUCT_0032
/* FP:tests.rs-0064 */ struct Shared < T : Write > { data : Arc < Mutex < T > > , }
/* FP:tests.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_IMPL_0033
/* FP:tests.rs-0066 */ impl < T : Write > WriteColor for Shared < T > { fn supports_color (& self) -> bool { false } fn set_color (& mut self , _spec : & termcolor :: ColorSpec) -> io :: Result < () > { Ok (()) } fn reset (& mut self) -> io :: Result < () > { Ok (()) } }
/* FP:tests.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_IMPL_0034
/* FP:tests.rs-0068 */ impl < T : Write > Write for Shared < T > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . data . lock () . unwrap () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . data . lock () . unwrap () . flush () } }
/* FP:tests.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0035
/* FP:tests.rs-0070 */ # [allow (rustc :: untranslatable_diagnostic)] fn test_harness (file_text : & str , span_labels : Vec < SpanLabel > , notes : Vec < (Option < (Position , Position) > , & 'static str) > , expected_output_ascii : & str , expected_output_unicode : & str ,) { create_default_session_globals_then (| | { for (theme , expected_output) in [(OutputTheme :: Ascii , expected_output_ascii) , (OutputTheme :: Unicode , expected_output_unicode) ,] { let (dcx , source_map , output) = create_test_handler (theme) ; source_map . new_source_file (Path :: new ("test.rs") . to_owned () . into () , file_text . to_owned ()) ; let primary_span = make_span (& file_text , & span_labels [0] . start , & span_labels [0] . end) ; let mut msp = MultiSpan :: from_span (primary_span) ; for span_label in & span_labels { let span = make_span (& file_text , & span_label . start , & span_label . end) ; msp . push_span_label (span , span_label . label) ; println ! ("span: {:?} label: {:?}" , span , span_label . label) ; println ! ("text: {:?}" , source_map . span_to_snippet (span)) ; } let mut err = dcx . handle () . struct_span_err (msp , "foo") ; for (position , note) in & notes { if let Some ((start , end)) = position { let span = make_span (& file_text , & start , & end) ; err . span_note (span , * note) ; } else { err . note (* note) ; } } err . emit () ; assert ! (expected_output . chars () . next () == Some ('\n') , "expected output should begin with newline") ; let expected_output = & expected_output [1 ..] ; let bytes = output . lock () . unwrap () ; let actual_output = str :: from_utf8 (& bytes) . unwrap () ; println ! ("expected output:\n------\n{}------" , expected_output) ; println ! ("actual output:\n------\n{}------" , actual_output) ; assert ! (expected_output == actual_output) } }) }
/* FP:tests.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0036
/* FP:tests.rs-0072 */ fn make_span (file_text : & str , start : & Position , end : & Position) -> Span { let start = make_pos (file_text , start) ; let end = make_pos (file_text , end) + end . string . len () ; assert ! (start <= end) ; Span :: with_root_ctxt (BytePos (start as u32) , BytePos (end as u32)) }
/* FP:tests.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0037
/* FP:tests.rs-0074 */ fn make_pos (file_text : & str , pos : & Position) -> usize { let mut remainder = file_text ; let mut offset = 0 ; for _ in 0 .. pos . count { if let Some (n) = remainder . find (& pos . string) { offset += n ; remainder = & remainder [n + 1 ..] ; } else { panic ! ("failed to find {} instances of {:?} in {:?}" , pos . count , pos . string , file_text) ; } } offset }
/* FP:tests.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0038
/* FP:tests.rs-0076 */ # [test] fn ends_on_col0 () { test_harness (r#"
/* FP:tests.rs-0077 */ fn foo() {
/* FP:tests.rs-0078 */ }
/* FP:tests.rs-0079 */ "# , vec ! [SpanLabel { start : Position { string : "{" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "test" , }] , vec ! [] , r#"
/* FP:tests.rs-0080 */ error: foo
/* FP:tests.rs-0081 */  --> test.rs:2:10
/* FP:tests.rs-0082 */   |
/* FP:tests.rs-0083 */ 2 |   fn foo() {
/* FP:tests.rs-0084 */   |  __________^
/* FP:tests.rs-0085 */ 3 | | }
/* FP:tests.rs-0086 */   | |_^ test
/* FP:tests.rs-0087 */ 
/* FP:tests.rs-0088 */ "# , r#"
/* FP:tests.rs-0089 */ error: foo
/* FP:tests.rs-0090 */   ╭▸ test.rs:2:10
/* FP:tests.rs-0091 */   │
/* FP:tests.rs-0092 */ 2 │   fn foo() {
/* FP:tests.rs-0093 */   │ ┏━━━━━━━━━━┛
/* FP:tests.rs-0094 */ 3 │ ┃ }
/* FP:tests.rs-0095 */   ╰╴┗━┛ test
/* FP:tests.rs-0096 */ 
/* FP:tests.rs-0097 */ "# ,) ; }
/* FP:tests.rs-0098 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0039
/* FP:tests.rs-0099 */ # [test] fn ends_on_col2 () { test_harness (r#"
/* FP:tests.rs-0100 */ fn foo() {
/* FP:tests.rs-0101 */ 
/* FP:tests.rs-0102 */ 
/* FP:tests.rs-0103 */   }
/* FP:tests.rs-0104 */ "# , vec ! [SpanLabel { start : Position { string : "{" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "test" , }] , vec ! [] , r#"
/* FP:tests.rs-0105 */ error: foo
/* FP:tests.rs-0106 */  --> test.rs:2:10
/* FP:tests.rs-0107 */   |
/* FP:tests.rs-0108 */ 2 |   fn foo() {
/* FP:tests.rs-0109 */   |  __________^
/* FP:tests.rs-0110 */ ... |
/* FP:tests.rs-0111 */ 5 | |   }
/* FP:tests.rs-0112 */   | |___^ test
/* FP:tests.rs-0113 */ 
/* FP:tests.rs-0114 */ "# , r#"
/* FP:tests.rs-0115 */ error: foo
/* FP:tests.rs-0116 */   ╭▸ test.rs:2:10
/* FP:tests.rs-0117 */   │
/* FP:tests.rs-0118 */ 2 │   fn foo() {
/* FP:tests.rs-0119 */   │ ┏━━━━━━━━━━┛
/* FP:tests.rs-0120 */   ‡ ┃
/* FP:tests.rs-0121 */ 5 │ ┃   }
/* FP:tests.rs-0122 */   ╰╴┗━━━┛ test
/* FP:tests.rs-0123 */ 
/* FP:tests.rs-0124 */ "# ,) ; }
/* FP:tests.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0040
/* FP:tests.rs-0126 */ # [test] fn non_nested () { test_harness (r#"
/* FP:tests.rs-0127 */ fn foo() {
/* FP:tests.rs-0128 */   X0 Y0
/* FP:tests.rs-0129 */   X1 Y1
/* FP:tests.rs-0130 */   X2 Y2
/* FP:tests.rs-0131 */ }
/* FP:tests.rs-0132 */ "# , vec ! [SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Y2" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0133 */ error: foo
/* FP:tests.rs-0134 */  --> test.rs:3:3
/* FP:tests.rs-0135 */   |
/* FP:tests.rs-0136 */ 3 |      X0 Y0
/* FP:tests.rs-0137 */   |  ____^  -
/* FP:tests.rs-0138 */   | | ______|
/* FP:tests.rs-0139 */ 4 | ||   X1 Y1
/* FP:tests.rs-0140 */ 5 | ||   X2 Y2
/* FP:tests.rs-0141 */   | ||____^__- `Y` is a good letter too
/* FP:tests.rs-0142 */   | |_____|
/* FP:tests.rs-0143 */   |       `X` is a good letter
/* FP:tests.rs-0144 */ 
/* FP:tests.rs-0145 */ "# , r#"
/* FP:tests.rs-0146 */ error: foo
/* FP:tests.rs-0147 */   ╭▸ test.rs:3:3
/* FP:tests.rs-0148 */   │
/* FP:tests.rs-0149 */ 3 │      X0 Y0
/* FP:tests.rs-0150 */   │ ┏━━━━┛  │
/* FP:tests.rs-0151 */   │ ┃┌──────┘
/* FP:tests.rs-0152 */ 4 │ ┃│   X1 Y1
/* FP:tests.rs-0153 */ 5 │ ┃│   X2 Y2
/* FP:tests.rs-0154 */   │ ┃└────╿──┘ `Y` is a good letter too
/* FP:tests.rs-0155 */   │ ┗━━━━━┥
/* FP:tests.rs-0156 */   ╰╴      `X` is a good letter
/* FP:tests.rs-0157 */ 
/* FP:tests.rs-0158 */ "# ,) ; }
/* FP:tests.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0041
/* FP:tests.rs-0160 */ # [test] fn nested () { test_harness (r#"
/* FP:tests.rs-0161 */ fn foo() {
/* FP:tests.rs-0162 */   X0 Y0
/* FP:tests.rs-0163 */   Y1 X1
/* FP:tests.rs-0164 */ }
/* FP:tests.rs-0165 */ "# , vec ! [SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X1" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Y1" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0166 */ error: foo
/* FP:tests.rs-0167 */  --> test.rs:3:3
/* FP:tests.rs-0168 */   |
/* FP:tests.rs-0169 */ 3 |      X0 Y0
/* FP:tests.rs-0170 */   |  ____^  -
/* FP:tests.rs-0171 */   | | ______|
/* FP:tests.rs-0172 */ 4 | ||   Y1 X1
/* FP:tests.rs-0173 */   | ||____-__^ `X` is a good letter
/* FP:tests.rs-0174 */   |  |____|
/* FP:tests.rs-0175 */   |       `Y` is a good letter too
/* FP:tests.rs-0176 */ 
/* FP:tests.rs-0177 */ "# , r#"
/* FP:tests.rs-0178 */ error: foo
/* FP:tests.rs-0179 */   ╭▸ test.rs:3:3
/* FP:tests.rs-0180 */   │
/* FP:tests.rs-0181 */ 3 │      X0 Y0
/* FP:tests.rs-0182 */   │ ┏━━━━┛  │
/* FP:tests.rs-0183 */   │ ┃┌──────┘
/* FP:tests.rs-0184 */ 4 │ ┃│   Y1 X1
/* FP:tests.rs-0185 */   │ ┗│━━━━│━━┛ `X` is a good letter
/* FP:tests.rs-0186 */   │  └────┤
/* FP:tests.rs-0187 */   ╰╴      `Y` is a good letter too
/* FP:tests.rs-0188 */ 
/* FP:tests.rs-0189 */ "# ,) ; }
/* FP:tests.rs-0190 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0042
/* FP:tests.rs-0191 */ # [test] fn multiline_and_normal_overlap () { test_harness (r#"
/* FP:tests.rs-0192 */ fn foo() {
/* FP:tests.rs-0193 */   X0 Y0 Z0
/* FP:tests.rs-0194 */   X1 Y1 Z1
/* FP:tests.rs-0195 */   X2 Y2 Z2
/* FP:tests.rs-0196 */   X3 Y3 Z3
/* FP:tests.rs-0197 */ }
/* FP:tests.rs-0198 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "Y0" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0199 */ error: foo
/* FP:tests.rs-0200 */  --> test.rs:3:6
/* FP:tests.rs-0201 */   |
/* FP:tests.rs-0202 */ 3 |     X0 Y0 Z0
/* FP:tests.rs-0203 */   |  ___---^-
/* FP:tests.rs-0204 */   | |   |
/* FP:tests.rs-0205 */   | |   `Y` is a good letter too
/* FP:tests.rs-0206 */ 4 | |   X1 Y1 Z1
/* FP:tests.rs-0207 */ 5 | |   X2 Y2 Z2
/* FP:tests.rs-0208 */   | |____^ `X` is a good letter
/* FP:tests.rs-0209 */ 
/* FP:tests.rs-0210 */ "# , r#"
/* FP:tests.rs-0211 */ error: foo
/* FP:tests.rs-0212 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0213 */   │
/* FP:tests.rs-0214 */ 3 │     X0 Y0 Z0
/* FP:tests.rs-0215 */   │ ┏━━━┬──┛─
/* FP:tests.rs-0216 */   │ ┃   │
/* FP:tests.rs-0217 */   │ ┃   `Y` is a good letter too
/* FP:tests.rs-0218 */ 4 │ ┃   X1 Y1 Z1
/* FP:tests.rs-0219 */ 5 │ ┃   X2 Y2 Z2
/* FP:tests.rs-0220 */   ╰╴┗━━━━┛ `X` is a good letter
/* FP:tests.rs-0221 */ 
/* FP:tests.rs-0222 */ "# ,) ; }
/* FP:tests.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0043
/* FP:tests.rs-0224 */ # [test] fn different_overlap () { test_harness (r#"
/* FP:tests.rs-0225 */ fn foo() {
/* FP:tests.rs-0226 */   X0 Y0 Z0
/* FP:tests.rs-0227 */   X1 Y1 Z1
/* FP:tests.rs-0228 */   X2 Y2 Z2
/* FP:tests.rs-0229 */   X3 Y3 Z3
/* FP:tests.rs-0230 */ }
/* FP:tests.rs-0231 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Z1" , count : 1 } , end : Position { string : "X3" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0232 */ error: foo
/* FP:tests.rs-0233 */  --> test.rs:3:6
/* FP:tests.rs-0234 */   |
/* FP:tests.rs-0235 */ 3 |      X0 Y0 Z0
/* FP:tests.rs-0236 */   |  _______^
/* FP:tests.rs-0237 */ 4 | |    X1 Y1 Z1
/* FP:tests.rs-0238 */   | | _________-
/* FP:tests.rs-0239 */ 5 | ||   X2 Y2 Z2
/* FP:tests.rs-0240 */   | ||____^ `X` is a good letter
/* FP:tests.rs-0241 */ 6 |  |   X3 Y3 Z3
/* FP:tests.rs-0242 */   |  |____- `Y` is a good letter too
/* FP:tests.rs-0243 */ 
/* FP:tests.rs-0244 */ "# , r#"
/* FP:tests.rs-0245 */ error: foo
/* FP:tests.rs-0246 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0247 */   │
/* FP:tests.rs-0248 */ 3 │      X0 Y0 Z0
/* FP:tests.rs-0249 */   │ ┏━━━━━━━┛
/* FP:tests.rs-0250 */ 4 │ ┃    X1 Y1 Z1
/* FP:tests.rs-0251 */   │ ┃┌─────────┘
/* FP:tests.rs-0252 */ 5 │ ┃│   X2 Y2 Z2
/* FP:tests.rs-0253 */   │ ┗│━━━━┛ `X` is a good letter
/* FP:tests.rs-0254 */ 6 │  │   X3 Y3 Z3
/* FP:tests.rs-0255 */   ╰╴ └────┘ `Y` is a good letter too
/* FP:tests.rs-0256 */ 
/* FP:tests.rs-0257 */ "# ,) ; }
/* FP:tests.rs-0258 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0044
/* FP:tests.rs-0259 */ # [test] fn different_note_1 () { test_harness (r#"
/* FP:tests.rs-0260 */ fn foo() {
/* FP:tests.rs-0261 */   X0 Y0 Z0
/* FP:tests.rs-0262 */   X1 Y1 Z1
/* FP:tests.rs-0263 */   X2 Y2 Z2
/* FP:tests.rs-0264 */   X3 Y3 Z3
/* FP:tests.rs-0265 */ }
/* FP:tests.rs-0266 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar")] , r#"
/* FP:tests.rs-0267 */ error: foo
/* FP:tests.rs-0268 */  --> test.rs:3:6
/* FP:tests.rs-0269 */   |
/* FP:tests.rs-0270 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0271 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0272 */   |
/* FP:tests.rs-0273 */   = note: bar
/* FP:tests.rs-0274 */ 
/* FP:tests.rs-0275 */ "# , r#"
/* FP:tests.rs-0276 */ error: foo
/* FP:tests.rs-0277 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0278 */   │
/* FP:tests.rs-0279 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0280 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0281 */   │
/* FP:tests.rs-0282 */   ╰ note: bar
/* FP:tests.rs-0283 */ 
/* FP:tests.rs-0284 */ "# ,) ; }
/* FP:tests.rs-0285 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0045
/* FP:tests.rs-0286 */ # [test] fn different_note_2 () { test_harness (r#"
/* FP:tests.rs-0287 */ fn foo() {
/* FP:tests.rs-0288 */   X0 Y0 Z0
/* FP:tests.rs-0289 */   X1 Y1 Z1
/* FP:tests.rs-0290 */   X2 Y2 Z2
/* FP:tests.rs-0291 */   X3 Y3 Z3
/* FP:tests.rs-0292 */ }
/* FP:tests.rs-0293 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar") , (None , "qux")] , r#"
/* FP:tests.rs-0294 */ error: foo
/* FP:tests.rs-0295 */  --> test.rs:3:6
/* FP:tests.rs-0296 */   |
/* FP:tests.rs-0297 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0298 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0299 */   |
/* FP:tests.rs-0300 */   = note: bar
/* FP:tests.rs-0301 */   = note: qux
/* FP:tests.rs-0302 */ 
/* FP:tests.rs-0303 */ "# , r#"
/* FP:tests.rs-0304 */ error: foo
/* FP:tests.rs-0305 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0306 */   │
/* FP:tests.rs-0307 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0308 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0309 */   │
/* FP:tests.rs-0310 */   ├ note: bar
/* FP:tests.rs-0311 */   ╰ note: qux
/* FP:tests.rs-0312 */ 
/* FP:tests.rs-0313 */ "# ,) ; }
/* FP:tests.rs-0314 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0046
/* FP:tests.rs-0315 */ # [test] fn different_note_3 () { test_harness (r#"
/* FP:tests.rs-0316 */ fn foo() {
/* FP:tests.rs-0317 */   X0 Y0 Z0
/* FP:tests.rs-0318 */   X1 Y1 Z1
/* FP:tests.rs-0319 */   X2 Y2 Z2
/* FP:tests.rs-0320 */   X3 Y3 Z3
/* FP:tests.rs-0321 */ }
/* FP:tests.rs-0322 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar") , (None , "baz") , (None , "qux")] , r#"
/* FP:tests.rs-0323 */ error: foo
/* FP:tests.rs-0324 */  --> test.rs:3:6
/* FP:tests.rs-0325 */   |
/* FP:tests.rs-0326 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0327 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0328 */   |
/* FP:tests.rs-0329 */   = note: bar
/* FP:tests.rs-0330 */   = note: baz
/* FP:tests.rs-0331 */   = note: qux
/* FP:tests.rs-0332 */ 
/* FP:tests.rs-0333 */ "# , r#"
/* FP:tests.rs-0334 */ error: foo
/* FP:tests.rs-0335 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0336 */   │
/* FP:tests.rs-0337 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0338 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0339 */   │
/* FP:tests.rs-0340 */   ├ note: bar
/* FP:tests.rs-0341 */   ├ note: baz
/* FP:tests.rs-0342 */   ╰ note: qux
/* FP:tests.rs-0343 */ 
/* FP:tests.rs-0344 */ "# ,) ; }
/* FP:tests.rs-0345 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0047
/* FP:tests.rs-0346 */ # [test] fn different_note_spanned_1 () { test_harness (r#"
/* FP:tests.rs-0347 */ fn foo() {
/* FP:tests.rs-0348 */   X0 Y0 Z0
/* FP:tests.rs-0349 */   X1 Y1 Z1
/* FP:tests.rs-0350 */   X2 Y2 Z2
/* FP:tests.rs-0351 */   X3 Y3 Z3
/* FP:tests.rs-0352 */ }
/* FP:tests.rs-0353 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,)] , r#"
/* FP:tests.rs-0354 */ error: foo
/* FP:tests.rs-0355 */  --> test.rs:3:6
/* FP:tests.rs-0356 */   |
/* FP:tests.rs-0357 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0358 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0359 */   |
/* FP:tests.rs-0360 */ note: bar
/* FP:tests.rs-0361 */  --> test.rs:4:3
/* FP:tests.rs-0362 */   |
/* FP:tests.rs-0363 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0364 */   |   ^^^^^^^^
/* FP:tests.rs-0365 */ 
/* FP:tests.rs-0366 */ "# , r#"
/* FP:tests.rs-0367 */ error: foo
/* FP:tests.rs-0368 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0369 */   │
/* FP:tests.rs-0370 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0371 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0372 */   ╰╴
/* FP:tests.rs-0373 */ note: bar
/* FP:tests.rs-0374 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0375 */   │
/* FP:tests.rs-0376 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0377 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0378 */ 
/* FP:tests.rs-0379 */ "# ,) ; }
/* FP:tests.rs-0380 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0048
/* FP:tests.rs-0381 */ # [test] fn different_note_spanned_2 () { test_harness (r#"
/* FP:tests.rs-0382 */ fn foo() {
/* FP:tests.rs-0383 */   X0 Y0 Z0
/* FP:tests.rs-0384 */   X1 Y1 Z1
/* FP:tests.rs-0385 */   X2 Y2 Z2
/* FP:tests.rs-0386 */   X3 Y3 Z3
/* FP:tests.rs-0387 */ }
/* FP:tests.rs-0388 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,) , (Some ((Position { string : "X2" , count : 1 } , Position { string : "Y2" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0389 */ error: foo
/* FP:tests.rs-0390 */  --> test.rs:3:6
/* FP:tests.rs-0391 */   |
/* FP:tests.rs-0392 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0393 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0394 */   |
/* FP:tests.rs-0395 */ note: bar
/* FP:tests.rs-0396 */  --> test.rs:4:3
/* FP:tests.rs-0397 */   |
/* FP:tests.rs-0398 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0399 */   |   ^^^^^^^^
/* FP:tests.rs-0400 */ note: qux
/* FP:tests.rs-0401 */  --> test.rs:5:3
/* FP:tests.rs-0402 */   |
/* FP:tests.rs-0403 */ 5 |   X2 Y2 Z2
/* FP:tests.rs-0404 */   |   ^^^^^
/* FP:tests.rs-0405 */ 
/* FP:tests.rs-0406 */ "# , r#"
/* FP:tests.rs-0407 */ error: foo
/* FP:tests.rs-0408 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0409 */   │
/* FP:tests.rs-0410 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0411 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0412 */   ╰╴
/* FP:tests.rs-0413 */ note: bar
/* FP:tests.rs-0414 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0415 */   │
/* FP:tests.rs-0416 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0417 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0418 */ note: qux
/* FP:tests.rs-0419 */   ╭▸ test.rs:5:3
/* FP:tests.rs-0420 */   │
/* FP:tests.rs-0421 */ 5 │   X2 Y2 Z2
/* FP:tests.rs-0422 */   ╰╴  ━━━━━
/* FP:tests.rs-0423 */ 
/* FP:tests.rs-0424 */ "# ,) ; }
/* FP:tests.rs-0425 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0049
/* FP:tests.rs-0426 */ # [test] fn different_note_spanned_3 () { test_harness (r#"
/* FP:tests.rs-0427 */ fn foo() {
/* FP:tests.rs-0428 */   X0 Y0 Z0
/* FP:tests.rs-0429 */   X1 Y1 Z1
/* FP:tests.rs-0430 */   X2 Y2 Z2
/* FP:tests.rs-0431 */   X3 Y3 Z3
/* FP:tests.rs-0432 */ }
/* FP:tests.rs-0433 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,) , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "baz" ,) , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0434 */ error: foo
/* FP:tests.rs-0435 */  --> test.rs:3:6
/* FP:tests.rs-0436 */   |
/* FP:tests.rs-0437 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0438 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0439 */   |
/* FP:tests.rs-0440 */ note: bar
/* FP:tests.rs-0441 */  --> test.rs:4:3
/* FP:tests.rs-0442 */   |
/* FP:tests.rs-0443 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0444 */   |   ^^^^^^^^
/* FP:tests.rs-0445 */ note: baz
/* FP:tests.rs-0446 */  --> test.rs:4:3
/* FP:tests.rs-0447 */   |
/* FP:tests.rs-0448 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0449 */   |   ^^^^^^^^
/* FP:tests.rs-0450 */ note: qux
/* FP:tests.rs-0451 */  --> test.rs:4:3
/* FP:tests.rs-0452 */   |
/* FP:tests.rs-0453 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0454 */   |   ^^^^^^^^
/* FP:tests.rs-0455 */ 
/* FP:tests.rs-0456 */ "# , r#"
/* FP:tests.rs-0457 */ error: foo
/* FP:tests.rs-0458 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0459 */   │
/* FP:tests.rs-0460 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0461 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0462 */   ╰╴
/* FP:tests.rs-0463 */ note: bar
/* FP:tests.rs-0464 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0465 */   │
/* FP:tests.rs-0466 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0467 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0468 */ note: baz
/* FP:tests.rs-0469 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0470 */   │
/* FP:tests.rs-0471 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0472 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0473 */ note: qux
/* FP:tests.rs-0474 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0475 */   │
/* FP:tests.rs-0476 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0477 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0478 */ 
/* FP:tests.rs-0479 */ "# ,) ; }
/* FP:tests.rs-0480 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0050
/* FP:tests.rs-0481 */ # [test] fn different_note_spanned_4 () { test_harness (r#"
/* FP:tests.rs-0482 */ fn foo() {
/* FP:tests.rs-0483 */   X0 Y0 Z0
/* FP:tests.rs-0484 */   X1 Y1 Z1
/* FP:tests.rs-0485 */   X2 Y2 Z2
/* FP:tests.rs-0486 */   X3 Y3 Z3
/* FP:tests.rs-0487 */ }
/* FP:tests.rs-0488 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,) , (None , "qux") ,] , r#"
/* FP:tests.rs-0489 */ error: foo
/* FP:tests.rs-0490 */  --> test.rs:3:6
/* FP:tests.rs-0491 */   |
/* FP:tests.rs-0492 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0493 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0494 */   |
/* FP:tests.rs-0495 */ note: bar
/* FP:tests.rs-0496 */  --> test.rs:4:3
/* FP:tests.rs-0497 */   |
/* FP:tests.rs-0498 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0499 */   |   ^^^^^^^^
/* FP:tests.rs-0500 */   = note: qux
/* FP:tests.rs-0501 */ 
/* FP:tests.rs-0502 */ "# , r#"
/* FP:tests.rs-0503 */ error: foo
/* FP:tests.rs-0504 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0505 */   │
/* FP:tests.rs-0506 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0507 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0508 */   ╰╴
/* FP:tests.rs-0509 */ note: bar
/* FP:tests.rs-0510 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0511 */   │
/* FP:tests.rs-0512 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0513 */   │   ━━━━━━━━
/* FP:tests.rs-0514 */   ╰ note: qux
/* FP:tests.rs-0515 */ 
/* FP:tests.rs-0516 */ "# ,) ; }
/* FP:tests.rs-0517 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0051
/* FP:tests.rs-0518 */ # [test] fn different_note_spanned_5 () { test_harness (r#"
/* FP:tests.rs-0519 */ fn foo() {
/* FP:tests.rs-0520 */   X0 Y0 Z0
/* FP:tests.rs-0521 */   X1 Y1 Z1
/* FP:tests.rs-0522 */   X2 Y2 Z2
/* FP:tests.rs-0523 */   X3 Y3 Z3
/* FP:tests.rs-0524 */ }
/* FP:tests.rs-0525 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar") , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0526 */ error: foo
/* FP:tests.rs-0527 */  --> test.rs:3:6
/* FP:tests.rs-0528 */   |
/* FP:tests.rs-0529 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0530 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0531 */   |
/* FP:tests.rs-0532 */   = note: bar
/* FP:tests.rs-0533 */ note: qux
/* FP:tests.rs-0534 */  --> test.rs:4:3
/* FP:tests.rs-0535 */   |
/* FP:tests.rs-0536 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0537 */   |   ^^^^^^^^
/* FP:tests.rs-0538 */ 
/* FP:tests.rs-0539 */ "# , r#"
/* FP:tests.rs-0540 */ error: foo
/* FP:tests.rs-0541 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0542 */   │
/* FP:tests.rs-0543 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0544 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0545 */   │
/* FP:tests.rs-0546 */   ╰ note: bar
/* FP:tests.rs-0547 */ note: qux
/* FP:tests.rs-0548 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0549 */   │
/* FP:tests.rs-0550 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0551 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0552 */ 
/* FP:tests.rs-0553 */ "# ,) ; }
/* FP:tests.rs-0554 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0052
/* FP:tests.rs-0555 */ # [test] fn different_note_spanned_6 () { test_harness (r#"
/* FP:tests.rs-0556 */ fn foo() {
/* FP:tests.rs-0557 */   X0 Y0 Z0
/* FP:tests.rs-0558 */   X1 Y1 Z1
/* FP:tests.rs-0559 */   X2 Y2 Z2
/* FP:tests.rs-0560 */   X3 Y3 Z3
/* FP:tests.rs-0561 */ }
/* FP:tests.rs-0562 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar") , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "baz" ,) , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0563 */ error: foo
/* FP:tests.rs-0564 */  --> test.rs:3:6
/* FP:tests.rs-0565 */   |
/* FP:tests.rs-0566 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0567 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0568 */   |
/* FP:tests.rs-0569 */   = note: bar
/* FP:tests.rs-0570 */ note: baz
/* FP:tests.rs-0571 */  --> test.rs:4:3
/* FP:tests.rs-0572 */   |
/* FP:tests.rs-0573 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0574 */   |   ^^^^^^^^
/* FP:tests.rs-0575 */ note: qux
/* FP:tests.rs-0576 */  --> test.rs:4:3
/* FP:tests.rs-0577 */   |
/* FP:tests.rs-0578 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0579 */   |   ^^^^^^^^
/* FP:tests.rs-0580 */ 
/* FP:tests.rs-0581 */ "# , r#"
/* FP:tests.rs-0582 */ error: foo
/* FP:tests.rs-0583 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0584 */   │
/* FP:tests.rs-0585 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0586 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0587 */   │
/* FP:tests.rs-0588 */   ╰ note: bar
/* FP:tests.rs-0589 */ note: baz
/* FP:tests.rs-0590 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0591 */   │
/* FP:tests.rs-0592 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0593 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0594 */ note: qux
/* FP:tests.rs-0595 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0596 */   │
/* FP:tests.rs-0597 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0598 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0599 */ 
/* FP:tests.rs-0600 */ "# ,) ; }
/* FP:tests.rs-0601 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0053
/* FP:tests.rs-0602 */ # [test] fn different_note_spanned_7 () { test_harness (r#"
/* FP:tests.rs-0603 */ fn foo() {
/* FP:tests.rs-0604 */   X0 Y0 Z0
/* FP:tests.rs-0605 */   X1 Y1 Z1
/* FP:tests.rs-0606 */   X2 Y2 Z2
/* FP:tests.rs-0607 */   X3 Y3 Z3
/* FP:tests.rs-0608 */ }
/* FP:tests.rs-0609 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z3" , count : 1 })) , "bar" ,) , (None , "baz") , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0610 */ error: foo
/* FP:tests.rs-0611 */  --> test.rs:3:6
/* FP:tests.rs-0612 */   |
/* FP:tests.rs-0613 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0614 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0615 */   |
/* FP:tests.rs-0616 */ note: bar
/* FP:tests.rs-0617 */  --> test.rs:4:3
/* FP:tests.rs-0618 */   |
/* FP:tests.rs-0619 */ 4 | /   X1 Y1 Z1
/* FP:tests.rs-0620 */ 5 | |   X2 Y2 Z2
/* FP:tests.rs-0621 */ 6 | |   X3 Y3 Z3
/* FP:tests.rs-0622 */   | |__________^
/* FP:tests.rs-0623 */   = note: baz
/* FP:tests.rs-0624 */ note: qux
/* FP:tests.rs-0625 */  --> test.rs:4:3
/* FP:tests.rs-0626 */   |
/* FP:tests.rs-0627 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0628 */   |   ^^^^^^^^
/* FP:tests.rs-0629 */ 
/* FP:tests.rs-0630 */ "# , r#"
/* FP:tests.rs-0631 */ error: foo
/* FP:tests.rs-0632 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0633 */   │
/* FP:tests.rs-0634 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0635 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0636 */   ╰╴
/* FP:tests.rs-0637 */ note: bar
/* FP:tests.rs-0638 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0639 */   │
/* FP:tests.rs-0640 */ 4 │ ┏   X1 Y1 Z1
/* FP:tests.rs-0641 */ 5 │ ┃   X2 Y2 Z2
/* FP:tests.rs-0642 */ 6 │ ┃   X3 Y3 Z3
/* FP:tests.rs-0643 */   │ ┗━━━━━━━━━━┛
/* FP:tests.rs-0644 */   ╰ note: baz
/* FP:tests.rs-0645 */ note: qux
/* FP:tests.rs-0646 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0647 */   │
/* FP:tests.rs-0648 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0649 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0650 */ 
/* FP:tests.rs-0651 */ "# ,) ; }
/* FP:tests.rs-0652 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0054
/* FP:tests.rs-0653 */ # [test] fn different_note_spanned_8 () { test_harness (r#"
/* FP:tests.rs-0654 */ fn foo() {
/* FP:tests.rs-0655 */   X0 Y0 Z0
/* FP:tests.rs-0656 */   X1 Y1 Z1
/* FP:tests.rs-0657 */   X2 Y2 Z2
/* FP:tests.rs-0658 */   X3 Y3 Z3
/* FP:tests.rs-0659 */ }
/* FP:tests.rs-0660 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,) , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "baz" ,) , (None , "qux") ,] , r#"
/* FP:tests.rs-0661 */ error: foo
/* FP:tests.rs-0662 */  --> test.rs:3:6
/* FP:tests.rs-0663 */   |
/* FP:tests.rs-0664 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0665 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0666 */   |
/* FP:tests.rs-0667 */ note: bar
/* FP:tests.rs-0668 */  --> test.rs:4:3
/* FP:tests.rs-0669 */   |
/* FP:tests.rs-0670 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0671 */   |   ^^^^^^^^
/* FP:tests.rs-0672 */ note: baz
/* FP:tests.rs-0673 */  --> test.rs:4:3
/* FP:tests.rs-0674 */   |
/* FP:tests.rs-0675 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0676 */   |   ^^^^^^^^
/* FP:tests.rs-0677 */   = note: qux
/* FP:tests.rs-0678 */ 
/* FP:tests.rs-0679 */ "# , r#"
/* FP:tests.rs-0680 */ error: foo
/* FP:tests.rs-0681 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0682 */   │
/* FP:tests.rs-0683 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0684 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0685 */   ╰╴
/* FP:tests.rs-0686 */ note: bar
/* FP:tests.rs-0687 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0688 */   │
/* FP:tests.rs-0689 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0690 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0691 */ note: baz
/* FP:tests.rs-0692 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0693 */   │
/* FP:tests.rs-0694 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0695 */   │   ━━━━━━━━
/* FP:tests.rs-0696 */   ╰ note: qux
/* FP:tests.rs-0697 */ 
/* FP:tests.rs-0698 */ "# ,) ; }
/* FP:tests.rs-0699 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0055
/* FP:tests.rs-0700 */ # [test] fn different_note_spanned_9 () { test_harness (r#"
/* FP:tests.rs-0701 */ fn foo() {
/* FP:tests.rs-0702 */   X0 Y0 Z0
/* FP:tests.rs-0703 */   X1 Y1 Z1
/* FP:tests.rs-0704 */   X2 Y2 Z2
/* FP:tests.rs-0705 */   X3 Y3 Z3
/* FP:tests.rs-0706 */ }
/* FP:tests.rs-0707 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(None , "bar") , (None , "baz") , (Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "qux" ,) ,] , r#"
/* FP:tests.rs-0708 */ error: foo
/* FP:tests.rs-0709 */  --> test.rs:3:6
/* FP:tests.rs-0710 */   |
/* FP:tests.rs-0711 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0712 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0713 */   |
/* FP:tests.rs-0714 */   = note: bar
/* FP:tests.rs-0715 */   = note: baz
/* FP:tests.rs-0716 */ note: qux
/* FP:tests.rs-0717 */  --> test.rs:4:3
/* FP:tests.rs-0718 */   |
/* FP:tests.rs-0719 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0720 */   |   ^^^^^^^^
/* FP:tests.rs-0721 */ 
/* FP:tests.rs-0722 */ "# , r#"
/* FP:tests.rs-0723 */ error: foo
/* FP:tests.rs-0724 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0725 */   │
/* FP:tests.rs-0726 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0727 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0728 */   │
/* FP:tests.rs-0729 */   ├ note: bar
/* FP:tests.rs-0730 */   ╰ note: baz
/* FP:tests.rs-0731 */ note: qux
/* FP:tests.rs-0732 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0733 */   │
/* FP:tests.rs-0734 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0735 */   ╰╴  ━━━━━━━━
/* FP:tests.rs-0736 */ 
/* FP:tests.rs-0737 */ "# ,) ; }
/* FP:tests.rs-0738 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0056
/* FP:tests.rs-0739 */ # [test] fn different_note_spanned_10 () { test_harness (r#"
/* FP:tests.rs-0740 */ fn foo() {
/* FP:tests.rs-0741 */   X0 Y0 Z0
/* FP:tests.rs-0742 */   X1 Y1 Z1
/* FP:tests.rs-0743 */   X2 Y2 Z2
/* FP:tests.rs-0744 */   X3 Y3 Z3
/* FP:tests.rs-0745 */ }
/* FP:tests.rs-0746 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Z0" , count : 1 } , label : "`X` is a good letter" , }] , vec ! [(Some ((Position { string : "X1" , count : 1 } , Position { string : "Z1" , count : 1 })) , "bar" ,) , (None , "baz") , (None , "qux") ,] , r#"
/* FP:tests.rs-0747 */ error: foo
/* FP:tests.rs-0748 */  --> test.rs:3:6
/* FP:tests.rs-0749 */   |
/* FP:tests.rs-0750 */ 3 |   X0 Y0 Z0
/* FP:tests.rs-0751 */   |      ^^^^^ `X` is a good letter
/* FP:tests.rs-0752 */   |
/* FP:tests.rs-0753 */ note: bar
/* FP:tests.rs-0754 */  --> test.rs:4:3
/* FP:tests.rs-0755 */   |
/* FP:tests.rs-0756 */ 4 |   X1 Y1 Z1
/* FP:tests.rs-0757 */   |   ^^^^^^^^
/* FP:tests.rs-0758 */   = note: baz
/* FP:tests.rs-0759 */   = note: qux
/* FP:tests.rs-0760 */ 
/* FP:tests.rs-0761 */ "# , r#"
/* FP:tests.rs-0762 */ error: foo
/* FP:tests.rs-0763 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0764 */   │
/* FP:tests.rs-0765 */ 3 │   X0 Y0 Z0
/* FP:tests.rs-0766 */   │      ━━━━━ `X` is a good letter
/* FP:tests.rs-0767 */   ╰╴
/* FP:tests.rs-0768 */ note: bar
/* FP:tests.rs-0769 */   ╭▸ test.rs:4:3
/* FP:tests.rs-0770 */   │
/* FP:tests.rs-0771 */ 4 │   X1 Y1 Z1
/* FP:tests.rs-0772 */   │   ━━━━━━━━
/* FP:tests.rs-0773 */   ├ note: baz
/* FP:tests.rs-0774 */   ╰ note: qux
/* FP:tests.rs-0775 */ 
/* FP:tests.rs-0776 */ "# ,) ; }
/* FP:tests.rs-0777 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0057
/* FP:tests.rs-0778 */ # [test] fn triple_overlap () { test_harness (r#"
/* FP:tests.rs-0779 */ fn foo() {
/* FP:tests.rs-0780 */   X0 Y0 Z0
/* FP:tests.rs-0781 */   X1 Y1 Z1
/* FP:tests.rs-0782 */   X2 Y2 Z2
/* FP:tests.rs-0783 */ }
/* FP:tests.rs-0784 */ "# , vec ! [SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Y2" , count : 1 } , label : "`Y` is a good letter too" , } , SpanLabel { start : Position { string : "Z0" , count : 1 } , end : Position { string : "Z2" , count : 1 } , label : "`Z` label" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0785 */ error: foo
/* FP:tests.rs-0786 */  --> test.rs:3:3
/* FP:tests.rs-0787 */   |
/* FP:tests.rs-0788 */ 3 |       X0 Y0 Z0
/* FP:tests.rs-0789 */   |  _____^  -  -
/* FP:tests.rs-0790 */   | | _______|  |
/* FP:tests.rs-0791 */   | || _________|
/* FP:tests.rs-0792 */ 4 | |||   X1 Y1 Z1
/* FP:tests.rs-0793 */ 5 | |||   X2 Y2 Z2
/* FP:tests.rs-0794 */   | |||____^__-__- `Z` label
/* FP:tests.rs-0795 */   | ||_____|__|
/* FP:tests.rs-0796 */   | |______|  `Y` is a good letter too
/* FP:tests.rs-0797 */   |        `X` is a good letter
/* FP:tests.rs-0798 */ 
/* FP:tests.rs-0799 */ "# , r#"
/* FP:tests.rs-0800 */ error: foo
/* FP:tests.rs-0801 */   ╭▸ test.rs:3:3
/* FP:tests.rs-0802 */   │
/* FP:tests.rs-0803 */ 3 │       X0 Y0 Z0
/* FP:tests.rs-0804 */   │ ┏━━━━━┛  │  │
/* FP:tests.rs-0805 */   │ ┃┌───────┘  │
/* FP:tests.rs-0806 */   │ ┃│┌─────────┘
/* FP:tests.rs-0807 */ 4 │ ┃││   X1 Y1 Z1
/* FP:tests.rs-0808 */ 5 │ ┃││   X2 Y2 Z2
/* FP:tests.rs-0809 */   │ ┃│└────╿──│──┘ `Z` label
/* FP:tests.rs-0810 */   │ ┃└─────│──┤
/* FP:tests.rs-0811 */   │ ┗━━━━━━┥  `Y` is a good letter too
/* FP:tests.rs-0812 */   ╰╴       `X` is a good letter
/* FP:tests.rs-0813 */ 
/* FP:tests.rs-0814 */ "# ,) ; }
/* FP:tests.rs-0815 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0058
/* FP:tests.rs-0816 */ # [test] fn triple_exact_overlap () { test_harness (r#"
/* FP:tests.rs-0817 */ fn foo() {
/* FP:tests.rs-0818 */   X0 Y0 Z0
/* FP:tests.rs-0819 */   X1 Y1 Z1
/* FP:tests.rs-0820 */   X2 Y2 Z2
/* FP:tests.rs-0821 */ }
/* FP:tests.rs-0822 */ "# , vec ! [SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`Y` is a good letter too" , } , SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X2" , count : 1 } , label : "`Z` label" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0823 */ error: foo
/* FP:tests.rs-0824 */  --> test.rs:3:3
/* FP:tests.rs-0825 */   |
/* FP:tests.rs-0826 */ 3 | /   X0 Y0 Z0
/* FP:tests.rs-0827 */ 4 | |   X1 Y1 Z1
/* FP:tests.rs-0828 */ 5 | |   X2 Y2 Z2
/* FP:tests.rs-0829 */   | |    ^
/* FP:tests.rs-0830 */   | |    |
/* FP:tests.rs-0831 */   | |    `X` is a good letter
/* FP:tests.rs-0832 */   | |____`Y` is a good letter too
/* FP:tests.rs-0833 */   |      `Z` label
/* FP:tests.rs-0834 */ 
/* FP:tests.rs-0835 */ "# , r#"
/* FP:tests.rs-0836 */ error: foo
/* FP:tests.rs-0837 */   ╭▸ test.rs:3:3
/* FP:tests.rs-0838 */   │
/* FP:tests.rs-0839 */ 3 │ ┏   X0 Y0 Z0
/* FP:tests.rs-0840 */ 4 │ ┃   X1 Y1 Z1
/* FP:tests.rs-0841 */ 5 │ ┃   X2 Y2 Z2
/* FP:tests.rs-0842 */   │ ┃    ╿
/* FP:tests.rs-0843 */   │ ┃    │
/* FP:tests.rs-0844 */   │ ┃    `X` is a good letter
/* FP:tests.rs-0845 */   │ ┗━━━━`Y` is a good letter too
/* FP:tests.rs-0846 */   ╰╴     `Z` label
/* FP:tests.rs-0847 */ 
/* FP:tests.rs-0848 */ "# ,) ; }
/* FP:tests.rs-0849 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0059
/* FP:tests.rs-0850 */ # [test] fn minimum_depth () { test_harness (r#"
/* FP:tests.rs-0851 */ fn foo() {
/* FP:tests.rs-0852 */   X0 Y0 Z0
/* FP:tests.rs-0853 */   X1 Y1 Z1
/* FP:tests.rs-0854 */   X2 Y2 Z2
/* FP:tests.rs-0855 */   X3 Y3 Z3
/* FP:tests.rs-0856 */ }
/* FP:tests.rs-0857 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "X1" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Y1" , count : 1 } , end : Position { string : "Z2" , count : 1 } , label : "`Y` is a good letter too" , } , SpanLabel { start : Position { string : "X2" , count : 1 } , end : Position { string : "Y3" , count : 1 } , label : "`Z`" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0858 */ error: foo
/* FP:tests.rs-0859 */  --> test.rs:3:6
/* FP:tests.rs-0860 */   |
/* FP:tests.rs-0861 */ 3 |      X0 Y0 Z0
/* FP:tests.rs-0862 */   |  _______^
/* FP:tests.rs-0863 */ 4 | |    X1 Y1 Z1
/* FP:tests.rs-0864 */   | | ____^_-
/* FP:tests.rs-0865 */   | ||____|
/* FP:tests.rs-0866 */   |  |    `X` is a good letter
/* FP:tests.rs-0867 */ 5 |  |   X2 Y2 Z2
/* FP:tests.rs-0868 */   |  |___-______- `Y` is a good letter too
/* FP:tests.rs-0869 */   |   ___|
/* FP:tests.rs-0870 */   |  |
/* FP:tests.rs-0871 */ 6 |  |   X3 Y3 Z3
/* FP:tests.rs-0872 */   |  |_______- `Z`
/* FP:tests.rs-0873 */ 
/* FP:tests.rs-0874 */ "# , r#"
/* FP:tests.rs-0875 */ error: foo
/* FP:tests.rs-0876 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0877 */   │
/* FP:tests.rs-0878 */ 3 │      X0 Y0 Z0
/* FP:tests.rs-0879 */   │ ┏━━━━━━━┛
/* FP:tests.rs-0880 */ 4 │ ┃    X1 Y1 Z1
/* FP:tests.rs-0881 */   │ ┃┌────╿─┘
/* FP:tests.rs-0882 */   │ ┗│━━━━┥
/* FP:tests.rs-0883 */   │  │    `X` is a good letter
/* FP:tests.rs-0884 */ 5 │  │   X2 Y2 Z2
/* FP:tests.rs-0885 */   │  └───│──────┘ `Y` is a good letter too
/* FP:tests.rs-0886 */   │  ┌───┘
/* FP:tests.rs-0887 */   │  │
/* FP:tests.rs-0888 */ 6 │  │   X3 Y3 Z3
/* FP:tests.rs-0889 */   ╰╴ └───────┘ `Z`
/* FP:tests.rs-0890 */ 
/* FP:tests.rs-0891 */ "# ,) ; }
/* FP:tests.rs-0892 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0060
/* FP:tests.rs-0893 */ # [test] fn non_overlapping () { test_harness (r#"
/* FP:tests.rs-0894 */ fn foo() {
/* FP:tests.rs-0895 */   X0 Y0 Z0
/* FP:tests.rs-0896 */   X1 Y1 Z1
/* FP:tests.rs-0897 */   X2 Y2 Z2
/* FP:tests.rs-0898 */   X3 Y3 Z3
/* FP:tests.rs-0899 */ }
/* FP:tests.rs-0900 */ "# , vec ! [SpanLabel { start : Position { string : "X0" , count : 1 } , end : Position { string : "X1" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Y2" , count : 1 } , end : Position { string : "Z3" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0901 */ error: foo
/* FP:tests.rs-0902 */  --> test.rs:3:3
/* FP:tests.rs-0903 */   |
/* FP:tests.rs-0904 */ 3 | /   X0 Y0 Z0
/* FP:tests.rs-0905 */ 4 | |   X1 Y1 Z1
/* FP:tests.rs-0906 */   | |____^ `X` is a good letter
/* FP:tests.rs-0907 */ 5 |     X2 Y2 Z2
/* FP:tests.rs-0908 */   |  ______-
/* FP:tests.rs-0909 */ 6 | |   X3 Y3 Z3
/* FP:tests.rs-0910 */   | |__________- `Y` is a good letter too
/* FP:tests.rs-0911 */ 
/* FP:tests.rs-0912 */ "# , r#"
/* FP:tests.rs-0913 */ error: foo
/* FP:tests.rs-0914 */   ╭▸ test.rs:3:3
/* FP:tests.rs-0915 */   │
/* FP:tests.rs-0916 */ 3 │ ┏   X0 Y0 Z0
/* FP:tests.rs-0917 */ 4 │ ┃   X1 Y1 Z1
/* FP:tests.rs-0918 */   │ ┗━━━━┛ `X` is a good letter
/* FP:tests.rs-0919 */ 5 │     X2 Y2 Z2
/* FP:tests.rs-0920 */   │ ┌──────┘
/* FP:tests.rs-0921 */ 6 │ │   X3 Y3 Z3
/* FP:tests.rs-0922 */   ╰╴└──────────┘ `Y` is a good letter too
/* FP:tests.rs-0923 */ 
/* FP:tests.rs-0924 */ "# ,) ; }
/* FP:tests.rs-0925 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0061
/* FP:tests.rs-0926 */ # [test] fn overlapping_start_and_end () { test_harness (r#"
/* FP:tests.rs-0927 */ fn foo() {
/* FP:tests.rs-0928 */   X0 Y0 Z0
/* FP:tests.rs-0929 */   X1 Y1 Z1
/* FP:tests.rs-0930 */   X2 Y2 Z2
/* FP:tests.rs-0931 */   X3 Y3 Z3
/* FP:tests.rs-0932 */ }
/* FP:tests.rs-0933 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "X1" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Z1" , count : 1 } , end : Position { string : "Z3" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0934 */ error: foo
/* FP:tests.rs-0935 */  --> test.rs:3:6
/* FP:tests.rs-0936 */   |
/* FP:tests.rs-0937 */ 3 |      X0 Y0 Z0
/* FP:tests.rs-0938 */   |  _______^
/* FP:tests.rs-0939 */ 4 | |    X1 Y1 Z1
/* FP:tests.rs-0940 */   | | ____^____-
/* FP:tests.rs-0941 */   | ||____|
/* FP:tests.rs-0942 */   |  |    `X` is a good letter
/* FP:tests.rs-0943 */ 5 |  |   X2 Y2 Z2
/* FP:tests.rs-0944 */ 6 |  |   X3 Y3 Z3
/* FP:tests.rs-0945 */   |  |__________- `Y` is a good letter too
/* FP:tests.rs-0946 */ 
/* FP:tests.rs-0947 */ "# , r#"
/* FP:tests.rs-0948 */ error: foo
/* FP:tests.rs-0949 */   ╭▸ test.rs:3:6
/* FP:tests.rs-0950 */   │
/* FP:tests.rs-0951 */ 3 │      X0 Y0 Z0
/* FP:tests.rs-0952 */   │ ┏━━━━━━━┛
/* FP:tests.rs-0953 */ 4 │ ┃    X1 Y1 Z1
/* FP:tests.rs-0954 */   │ ┃┌────╿────┘
/* FP:tests.rs-0955 */   │ ┗│━━━━┥
/* FP:tests.rs-0956 */   │  │    `X` is a good letter
/* FP:tests.rs-0957 */ 5 │  │   X2 Y2 Z2
/* FP:tests.rs-0958 */ 6 │  │   X3 Y3 Z3
/* FP:tests.rs-0959 */   ╰╴ └──────────┘ `Y` is a good letter too
/* FP:tests.rs-0960 */ 
/* FP:tests.rs-0961 */ "# ,) ; }
/* FP:tests.rs-0962 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0062
/* FP:tests.rs-0963 */ # [test] fn multiple_labels_primary_without_message () { test_harness (r#"
/* FP:tests.rs-0964 */ fn foo() {
/* FP:tests.rs-0965 */   a { b { c } d }
/* FP:tests.rs-0966 */ }
/* FP:tests.rs-0967 */ "# , vec ! [SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "`a` is a good letter" , } , SpanLabel { start : Position { string : "c" , count : 1 } , end : Position { string : "c" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-0968 */ error: foo
/* FP:tests.rs-0969 */  --> test.rs:3:7
/* FP:tests.rs-0970 */   |
/* FP:tests.rs-0971 */ 3 |   a { b { c } d }
/* FP:tests.rs-0972 */   |   ----^^^^-^^-- `a` is a good letter
/* FP:tests.rs-0973 */ 
/* FP:tests.rs-0974 */ "# , r#"
/* FP:tests.rs-0975 */ error: foo
/* FP:tests.rs-0976 */   ╭▸ test.rs:3:7
/* FP:tests.rs-0977 */   │
/* FP:tests.rs-0978 */ 3 │   a { b { c } d }
/* FP:tests.rs-0979 */   ╰╴  ────━━━━─━━── `a` is a good letter
/* FP:tests.rs-0980 */ 
/* FP:tests.rs-0981 */ "# ,) ; }
/* FP:tests.rs-0982 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0063
/* FP:tests.rs-0983 */ # [test] fn multiline_notes () { test_harness (r#"
/* FP:tests.rs-0984 */ fn foo() {
/* FP:tests.rs-0985 */   a { b { c } d }
/* FP:tests.rs-0986 */ }
/* FP:tests.rs-0987 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "`a` is a good letter" , }] , vec ! [(None , "foo\nbar") , (None , "foo\nbar")] , r#"
/* FP:tests.rs-0988 */ error: foo
/* FP:tests.rs-0989 */  --> test.rs:3:3
/* FP:tests.rs-0990 */   |
/* FP:tests.rs-0991 */ 3 |   a { b { c } d }
/* FP:tests.rs-0992 */   |   ^^^^^^^^^^^^^ `a` is a good letter
/* FP:tests.rs-0993 */   |
/* FP:tests.rs-0994 */   = note: foo
/* FP:tests.rs-0995 */           bar
/* FP:tests.rs-0996 */   = note: foo
/* FP:tests.rs-0997 */           bar
/* FP:tests.rs-0998 */ 
/* FP:tests.rs-0999 */ "# , r#"
/* FP:tests.rs-1000 */ error: foo
/* FP:tests.rs-1001 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1002 */   │
/* FP:tests.rs-1003 */ 3 │   a { b { c } d }
/* FP:tests.rs-1004 */   │   ━━━━━━━━━━━━━ `a` is a good letter
/* FP:tests.rs-1005 */   │
/* FP:tests.rs-1006 */   ├ note: foo
/* FP:tests.rs-1007 */   │       bar
/* FP:tests.rs-1008 */   ╰ note: foo
/* FP:tests.rs-1009 */           bar
/* FP:tests.rs-1010 */ 
/* FP:tests.rs-1011 */ "# ,) ; }
/* FP:tests.rs-1012 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0064
/* FP:tests.rs-1013 */ # [test] fn multiple_labels_secondary_without_message () { test_harness (r#"
/* FP:tests.rs-1014 */ fn foo() {
/* FP:tests.rs-1015 */   a { b { c } d }
/* FP:tests.rs-1016 */ }
/* FP:tests.rs-1017 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "`a` is a good letter" , } , SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1018 */ error: foo
/* FP:tests.rs-1019 */  --> test.rs:3:3
/* FP:tests.rs-1020 */   |
/* FP:tests.rs-1021 */ 3 |   a { b { c } d }
/* FP:tests.rs-1022 */   |   ^^^^-------^^ `a` is a good letter
/* FP:tests.rs-1023 */ 
/* FP:tests.rs-1024 */ "# , r#"
/* FP:tests.rs-1025 */ error: foo
/* FP:tests.rs-1026 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1027 */   │
/* FP:tests.rs-1028 */ 3 │   a { b { c } d }
/* FP:tests.rs-1029 */   ╰╴  ━━━━───────━━ `a` is a good letter
/* FP:tests.rs-1030 */ 
/* FP:tests.rs-1031 */ "# ,) ; }
/* FP:tests.rs-1032 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0065
/* FP:tests.rs-1033 */ # [test] fn multiple_labels_primary_without_message_2 () { test_harness (r#"
/* FP:tests.rs-1034 */ fn foo() {
/* FP:tests.rs-1035 */   a { b { c } d }
/* FP:tests.rs-1036 */ }
/* FP:tests.rs-1037 */ "# , vec ! [SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "`b` is a good letter" , } , SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "c" , count : 1 } , end : Position { string : "c" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1038 */ error: foo
/* FP:tests.rs-1039 */  --> test.rs:3:7
/* FP:tests.rs-1040 */   |
/* FP:tests.rs-1041 */ 3 |   a { b { c } d }
/* FP:tests.rs-1042 */   |   ----^^^^-^^--
/* FP:tests.rs-1043 */   |       |
/* FP:tests.rs-1044 */   |       `b` is a good letter
/* FP:tests.rs-1045 */ 
/* FP:tests.rs-1046 */ "# , r#"
/* FP:tests.rs-1047 */ error: foo
/* FP:tests.rs-1048 */   ╭▸ test.rs:3:7
/* FP:tests.rs-1049 */   │
/* FP:tests.rs-1050 */ 3 │   a { b { c } d }
/* FP:tests.rs-1051 */   │   ────┯━━━─━━──
/* FP:tests.rs-1052 */   │       │
/* FP:tests.rs-1053 */   ╰╴      `b` is a good letter
/* FP:tests.rs-1054 */ 
/* FP:tests.rs-1055 */ "# ,) ; }
/* FP:tests.rs-1056 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0066
/* FP:tests.rs-1057 */ # [test] fn multiple_labels_secondary_without_message_2 () { test_harness (r#"
/* FP:tests.rs-1058 */ fn foo() {
/* FP:tests.rs-1059 */   a { b { c } d }
/* FP:tests.rs-1060 */ }
/* FP:tests.rs-1061 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "`b` is a good letter" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1062 */ error: foo
/* FP:tests.rs-1063 */  --> test.rs:3:3
/* FP:tests.rs-1064 */   |
/* FP:tests.rs-1065 */ 3 |   a { b { c } d }
/* FP:tests.rs-1066 */   |   ^^^^-------^^
/* FP:tests.rs-1067 */   |       |
/* FP:tests.rs-1068 */   |       `b` is a good letter
/* FP:tests.rs-1069 */ 
/* FP:tests.rs-1070 */ "# , r#"
/* FP:tests.rs-1071 */ error: foo
/* FP:tests.rs-1072 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1073 */   │
/* FP:tests.rs-1074 */ 3 │   a { b { c } d }
/* FP:tests.rs-1075 */   │   ━━━━┬──────━━
/* FP:tests.rs-1076 */   │       │
/* FP:tests.rs-1077 */   ╰╴      `b` is a good letter
/* FP:tests.rs-1078 */ 
/* FP:tests.rs-1079 */ "# ,) ; }
/* FP:tests.rs-1080 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0067
/* FP:tests.rs-1081 */ # [test] fn multiple_labels_secondary_without_message_3 () { test_harness (r#"
/* FP:tests.rs-1082 */ fn foo() {
/* FP:tests.rs-1083 */   a  bc  d
/* FP:tests.rs-1084 */ }
/* FP:tests.rs-1085 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "b" , count : 1 } , label : "`a` is a good letter" , } , SpanLabel { start : Position { string : "c" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1086 */ error: foo
/* FP:tests.rs-1087 */  --> test.rs:3:3
/* FP:tests.rs-1088 */   |
/* FP:tests.rs-1089 */ 3 |   a  bc  d
/* FP:tests.rs-1090 */   |   ^^^^----
/* FP:tests.rs-1091 */   |   |
/* FP:tests.rs-1092 */   |   `a` is a good letter
/* FP:tests.rs-1093 */ 
/* FP:tests.rs-1094 */ "# , r#"
/* FP:tests.rs-1095 */ error: foo
/* FP:tests.rs-1096 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1097 */   │
/* FP:tests.rs-1098 */ 3 │   a  bc  d
/* FP:tests.rs-1099 */   │   ┯━━━────
/* FP:tests.rs-1100 */   │   │
/* FP:tests.rs-1101 */   ╰╴  `a` is a good letter
/* FP:tests.rs-1102 */ 
/* FP:tests.rs-1103 */ "# ,) ; }
/* FP:tests.rs-1104 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0068
/* FP:tests.rs-1105 */ # [test] fn multiple_labels_without_message () { test_harness (r#"
/* FP:tests.rs-1106 */ fn foo() {
/* FP:tests.rs-1107 */   a { b { c } d }
/* FP:tests.rs-1108 */ }
/* FP:tests.rs-1109 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1110 */ error: foo
/* FP:tests.rs-1111 */  --> test.rs:3:3
/* FP:tests.rs-1112 */   |
/* FP:tests.rs-1113 */ 3 |   a { b { c } d }
/* FP:tests.rs-1114 */   |   ^^^^-------^^
/* FP:tests.rs-1115 */ 
/* FP:tests.rs-1116 */ "# , r#"
/* FP:tests.rs-1117 */ error: foo
/* FP:tests.rs-1118 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1119 */   │
/* FP:tests.rs-1120 */ 3 │   a { b { c } d }
/* FP:tests.rs-1121 */   ╰╴  ━━━━───────━━
/* FP:tests.rs-1122 */ 
/* FP:tests.rs-1123 */ "# ,) ; }
/* FP:tests.rs-1124 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0069
/* FP:tests.rs-1125 */ # [test] fn multiple_labels_without_message_2 () { test_harness (r#"
/* FP:tests.rs-1126 */ fn foo() {
/* FP:tests.rs-1127 */   a { b { c } d }
/* FP:tests.rs-1128 */ }
/* FP:tests.rs-1129 */ "# , vec ! [SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , } , SpanLabel { start : Position { string : "c" , count : 1 } , end : Position { string : "c" , count : 1 } , label : "" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1130 */ error: foo
/* FP:tests.rs-1131 */  --> test.rs:3:7
/* FP:tests.rs-1132 */   |
/* FP:tests.rs-1133 */ 3 |   a { b { c } d }
/* FP:tests.rs-1134 */   |   ----^^^^-^^--
/* FP:tests.rs-1135 */ 
/* FP:tests.rs-1136 */ "# , r#"
/* FP:tests.rs-1137 */ error: foo
/* FP:tests.rs-1138 */   ╭▸ test.rs:3:7
/* FP:tests.rs-1139 */   │
/* FP:tests.rs-1140 */ 3 │   a { b { c } d }
/* FP:tests.rs-1141 */   ╰╴  ────━━━━─━━──
/* FP:tests.rs-1142 */ 
/* FP:tests.rs-1143 */ "# ,) ; }
/* FP:tests.rs-1144 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0070
/* FP:tests.rs-1145 */ # [test] fn multiple_labels_with_message () { test_harness (r#"
/* FP:tests.rs-1146 */ fn foo() {
/* FP:tests.rs-1147 */   a { b { c } d }
/* FP:tests.rs-1148 */ }
/* FP:tests.rs-1149 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "`a` is a good letter" , } , SpanLabel { start : Position { string : "b" , count : 1 } , end : Position { string : "}" , count : 1 } , label : "`b` is a good letter" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1150 */ error: foo
/* FP:tests.rs-1151 */  --> test.rs:3:3
/* FP:tests.rs-1152 */   |
/* FP:tests.rs-1153 */ 3 |   a { b { c } d }
/* FP:tests.rs-1154 */   |   ^^^^-------^^
/* FP:tests.rs-1155 */   |   |   |
/* FP:tests.rs-1156 */   |   |   `b` is a good letter
/* FP:tests.rs-1157 */   |   `a` is a good letter
/* FP:tests.rs-1158 */ 
/* FP:tests.rs-1159 */ "# , r#"
/* FP:tests.rs-1160 */ error: foo
/* FP:tests.rs-1161 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1162 */   │
/* FP:tests.rs-1163 */ 3 │   a { b { c } d }
/* FP:tests.rs-1164 */   │   ┯━━━┬──────━━
/* FP:tests.rs-1165 */   │   │   │
/* FP:tests.rs-1166 */   │   │   `b` is a good letter
/* FP:tests.rs-1167 */   ╰╴  `a` is a good letter
/* FP:tests.rs-1168 */ 
/* FP:tests.rs-1169 */ "# ,) ; }
/* FP:tests.rs-1170 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0071
/* FP:tests.rs-1171 */ # [test] fn single_label_with_message () { test_harness (r#"
/* FP:tests.rs-1172 */ fn foo() {
/* FP:tests.rs-1173 */   a { b { c } d }
/* FP:tests.rs-1174 */ }
/* FP:tests.rs-1175 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "`a` is a good letter" , }] , vec ! [] , r#"
/* FP:tests.rs-1176 */ error: foo
/* FP:tests.rs-1177 */  --> test.rs:3:3
/* FP:tests.rs-1178 */   |
/* FP:tests.rs-1179 */ 3 |   a { b { c } d }
/* FP:tests.rs-1180 */   |   ^^^^^^^^^^^^^ `a` is a good letter
/* FP:tests.rs-1181 */ 
/* FP:tests.rs-1182 */ "# , r#"
/* FP:tests.rs-1183 */ error: foo
/* FP:tests.rs-1184 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1185 */   │
/* FP:tests.rs-1186 */ 3 │   a { b { c } d }
/* FP:tests.rs-1187 */   ╰╴  ━━━━━━━━━━━━━ `a` is a good letter
/* FP:tests.rs-1188 */ 
/* FP:tests.rs-1189 */ "# ,) ; }
/* FP:tests.rs-1190 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0072
/* FP:tests.rs-1191 */ # [test] fn single_label_without_message () { test_harness (r#"
/* FP:tests.rs-1192 */ fn foo() {
/* FP:tests.rs-1193 */   a { b { c } d }
/* FP:tests.rs-1194 */ }
/* FP:tests.rs-1195 */ "# , vec ! [SpanLabel { start : Position { string : "a" , count : 1 } , end : Position { string : "d" , count : 1 } , label : "" , }] , vec ! [] , r#"
/* FP:tests.rs-1196 */ error: foo
/* FP:tests.rs-1197 */  --> test.rs:3:3
/* FP:tests.rs-1198 */   |
/* FP:tests.rs-1199 */ 3 |   a { b { c } d }
/* FP:tests.rs-1200 */   |   ^^^^^^^^^^^^^
/* FP:tests.rs-1201 */ 
/* FP:tests.rs-1202 */ "# , r#"
/* FP:tests.rs-1203 */ error: foo
/* FP:tests.rs-1204 */   ╭▸ test.rs:3:3
/* FP:tests.rs-1205 */   │
/* FP:tests.rs-1206 */ 3 │   a { b { c } d }
/* FP:tests.rs-1207 */   ╰╴  ━━━━━━━━━━━━━
/* FP:tests.rs-1208 */ 
/* FP:tests.rs-1209 */ "# ,) ; }
/* FP:tests.rs-1210 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0073
/* FP:tests.rs-1211 */ # [test] fn long_snippet () { test_harness (r#"
/* FP:tests.rs-1212 */ fn foo() {
/* FP:tests.rs-1213 */   X0 Y0 Z0
/* FP:tests.rs-1214 */   X1 Y1 Z1
/* FP:tests.rs-1215 */ 1
/* FP:tests.rs-1216 */ 2
/* FP:tests.rs-1217 */ 3
/* FP:tests.rs-1218 */ 4
/* FP:tests.rs-1219 */ 5
/* FP:tests.rs-1220 */ 6
/* FP:tests.rs-1221 */ 7
/* FP:tests.rs-1222 */ 8
/* FP:tests.rs-1223 */ 9
/* FP:tests.rs-1224 */ 10
/* FP:tests.rs-1225 */   X2 Y2 Z2
/* FP:tests.rs-1226 */   X3 Y3 Z3
/* FP:tests.rs-1227 */ }
/* FP:tests.rs-1228 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "X1" , count : 1 } , label : "`X` is a good letter" , } , SpanLabel { start : Position { string : "Z1" , count : 1 } , end : Position { string : "Z3" , count : 1 } , label : "`Y` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1229 */ error: foo
/* FP:tests.rs-1230 */   --> test.rs:3:6
/* FP:tests.rs-1231 */    |
/* FP:tests.rs-1232 */  3 |      X0 Y0 Z0
/* FP:tests.rs-1233 */    |  _______^
/* FP:tests.rs-1234 */  4 | |    X1 Y1 Z1
/* FP:tests.rs-1235 */    | | ____^____-
/* FP:tests.rs-1236 */    | ||____|
/* FP:tests.rs-1237 */    |  |    `X` is a good letter
/* FP:tests.rs-1238 */  5 |  | 1
/* FP:tests.rs-1239 */  6 |  | 2
/* FP:tests.rs-1240 */  7 |  | 3
/* FP:tests.rs-1241 */ ...   |
/* FP:tests.rs-1242 */ 15 |  |   X2 Y2 Z2
/* FP:tests.rs-1243 */ 16 |  |   X3 Y3 Z3
/* FP:tests.rs-1244 */    |  |__________- `Y` is a good letter too
/* FP:tests.rs-1245 */ 
/* FP:tests.rs-1246 */ "# , r#"
/* FP:tests.rs-1247 */ error: foo
/* FP:tests.rs-1248 */    ╭▸ test.rs:3:6
/* FP:tests.rs-1249 */    │
/* FP:tests.rs-1250 */  3 │      X0 Y0 Z0
/* FP:tests.rs-1251 */    │ ┏━━━━━━━┛
/* FP:tests.rs-1252 */  4 │ ┃    X1 Y1 Z1
/* FP:tests.rs-1253 */    │ ┃┌────╿────┘
/* FP:tests.rs-1254 */    │ ┗│━━━━┥
/* FP:tests.rs-1255 */    │  │    `X` is a good letter
/* FP:tests.rs-1256 */  5 │  │ 1
/* FP:tests.rs-1257 */  6 │  │ 2
/* FP:tests.rs-1258 */  7 │  │ 3
/* FP:tests.rs-1259 */    ‡  │
/* FP:tests.rs-1260 */ 15 │  │   X2 Y2 Z2
/* FP:tests.rs-1261 */ 16 │  │   X3 Y3 Z3
/* FP:tests.rs-1262 */    ╰╴ └──────────┘ `Y` is a good letter too
/* FP:tests.rs-1263 */ 
/* FP:tests.rs-1264 */ "# ,) ; }
/* FP:tests.rs-1265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0074
/* FP:tests.rs-1266 */ # [test] fn long_snippet_multiple_spans () { test_harness (r#"
/* FP:tests.rs-1267 */ fn foo() {
/* FP:tests.rs-1268 */   X0 Y0 Z0
/* FP:tests.rs-1269 */ 1
/* FP:tests.rs-1270 */ 2
/* FP:tests.rs-1271 */ 3
/* FP:tests.rs-1272 */   X1 Y1 Z1
/* FP:tests.rs-1273 */ 4
/* FP:tests.rs-1274 */ 5
/* FP:tests.rs-1275 */ 6
/* FP:tests.rs-1276 */   X2 Y2 Z2
/* FP:tests.rs-1277 */ 7
/* FP:tests.rs-1278 */ 8
/* FP:tests.rs-1279 */ 9
/* FP:tests.rs-1280 */ 10
/* FP:tests.rs-1281 */   X3 Y3 Z3
/* FP:tests.rs-1282 */ }
/* FP:tests.rs-1283 */ "# , vec ! [SpanLabel { start : Position { string : "Y0" , count : 1 } , end : Position { string : "Y3" , count : 1 } , label : "`Y` is a good letter" , } , SpanLabel { start : Position { string : "Z1" , count : 1 } , end : Position { string : "Z2" , count : 1 } , label : "`Z` is a good letter too" , } ,] , vec ! [] , r#"
/* FP:tests.rs-1284 */ error: foo
/* FP:tests.rs-1285 */   --> test.rs:3:6
/* FP:tests.rs-1286 */    |
/* FP:tests.rs-1287 */  3 |      X0 Y0 Z0
/* FP:tests.rs-1288 */    |  _______^
/* FP:tests.rs-1289 */  4 | |  1
/* FP:tests.rs-1290 */  5 | |  2
/* FP:tests.rs-1291 */  6 | |  3
/* FP:tests.rs-1292 */  7 | |    X1 Y1 Z1
/* FP:tests.rs-1293 */    | | _________-
/* FP:tests.rs-1294 */  8 | || 4
/* FP:tests.rs-1295 */  9 | || 5
/* FP:tests.rs-1296 */ 10 | || 6
/* FP:tests.rs-1297 */ 11 | ||   X2 Y2 Z2
/* FP:tests.rs-1298 */    | ||__________- `Z` is a good letter too
/* FP:tests.rs-1299 */ ...  |
/* FP:tests.rs-1300 */ 15 | |  10
/* FP:tests.rs-1301 */ 16 | |    X3 Y3 Z3
/* FP:tests.rs-1302 */    | |________^ `Y` is a good letter
/* FP:tests.rs-1303 */ 
/* FP:tests.rs-1304 */ "# , r#"
/* FP:tests.rs-1305 */ error: foo
/* FP:tests.rs-1306 */    ╭▸ test.rs:3:6
/* FP:tests.rs-1307 */    │
/* FP:tests.rs-1308 */  3 │      X0 Y0 Z0
/* FP:tests.rs-1309 */    │ ┏━━━━━━━┛
/* FP:tests.rs-1310 */  4 │ ┃  1
/* FP:tests.rs-1311 */  5 │ ┃  2
/* FP:tests.rs-1312 */  6 │ ┃  3
/* FP:tests.rs-1313 */  7 │ ┃    X1 Y1 Z1
/* FP:tests.rs-1314 */    │ ┃┌─────────┘
/* FP:tests.rs-1315 */  8 │ ┃│ 4
/* FP:tests.rs-1316 */  9 │ ┃│ 5
/* FP:tests.rs-1317 */ 10 │ ┃│ 6
/* FP:tests.rs-1318 */ 11 │ ┃│   X2 Y2 Z2
/* FP:tests.rs-1319 */    │ ┃└──────────┘ `Z` is a good letter too
/* FP:tests.rs-1320 */    ‡ ┃
/* FP:tests.rs-1321 */ 15 │ ┃  10
/* FP:tests.rs-1322 */ 16 │ ┃    X3 Y3 Z3
/* FP:tests.rs-1323 */    ╰╴┗━━━━━━━━┛ `Y` is a good letter
/* FP:tests.rs-1324 */ 
/* FP:tests.rs-1325 */ "# ,) ; }
/* FP:tests.rs-1326 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0075
/* FP:tests.rs-1327 */ # [doc = " Parses an item."] # [doc = ""] # [doc = " Returns `Ok(Some(item))` when successful, `Ok(None)` when no item was found, and `Err`"] # [doc = " when a syntax error occurred."] fn parse_item_from_source_str (name : FileName , source : String , psess : & ParseSess ,) -> PResult < '_ , Option < Box < ast :: Item > > > { unwrap_or_emit_fatal (new_parser_from_source_str (psess , name , source , StripTokens :: Nothing)) . parse_item (ForceCollect :: No) }
/* FP:tests.rs-1328 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0076
/* FP:tests.rs-1329 */ fn sp (a : u32 , b : u32) -> Span { Span :: with_root_ctxt (BytePos (a) , BytePos (b)) }
/* FP:tests.rs-1330 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0077
/* FP:tests.rs-1331 */ # [doc = " Parses a string, return an expression."] fn string_to_expr (source_str : String) -> Box < ast :: Expr > { with_error_checking_parse (source_str , & psess () , | p | p . parse_expr ()) }
/* FP:tests.rs-1332 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0078
/* FP:tests.rs-1333 */ # [doc = " Parses a string, returns an item."] fn string_to_item (source_str : String) -> Option < Box < ast :: Item > > { with_error_checking_parse (source_str , & psess () , | p | p . parse_item (ForceCollect :: No)) }
/* FP:tests.rs-1334 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0079
/* FP:tests.rs-1335 */ # [test] fn bad_path_expr_1 () { create_default_session_globals_then (| | { with_expected_parse_error ("::abc::def::return" , "expected identifier, found keyword `return`" , | p | p . parse_expr () ,) ; }) }
/* FP:tests.rs-1336 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0080
/* FP:tests.rs-1337 */ # [test] fn string_to_tts_macro () { create_default_session_globals_then (| | { let stream = string_to_stream ("macro_rules! zip (($a)=>($a))" . to_string ()) ; let tts = & stream . iter () . collect :: < Vec < _ > > () [..] ; match tts { [TokenTree :: Token (Token { kind : token :: Ident (name_macro_rules , IdentIsRaw :: No) , .. } , _ ,) , TokenTree :: Token (Token { kind : token :: Bang , .. } , _) , TokenTree :: Token (Token { kind : token :: Ident (name_zip , IdentIsRaw :: No) , .. } , _) , TokenTree :: Delimited (.. , macro_delim , macro_tts) ,] if name_macro_rules == & kw :: MacroRules && name_zip . as_str () == "zip" => { let tts = & macro_tts . iter () . collect :: < Vec < _ > > () ; match & tts [..] { [TokenTree :: Delimited (.. , first_delim , first_tts) , TokenTree :: Token (Token { kind : token :: FatArrow , .. } , _) , TokenTree :: Delimited (.. , second_delim , second_tts) ,] if macro_delim == & Delimiter :: Parenthesis => { let tts = & first_tts . iter () . collect :: < Vec < _ > > () ; match & tts [..] { [TokenTree :: Token (Token { kind : token :: Dollar , .. } , _) , TokenTree :: Token (Token { kind : token :: Ident (name , IdentIsRaw :: No) , .. } , _ ,) ,] if first_delim == & Delimiter :: Parenthesis && name . as_str () == "a" => { } _ => panic ! ("value 3: {:?} {:?}" , first_delim , first_tts) , } let tts = & second_tts . iter () . collect :: < Vec < _ > > () ; match & tts [..] { [TokenTree :: Token (Token { kind : token :: Dollar , .. } , _) , TokenTree :: Token (Token { kind : token :: Ident (name , IdentIsRaw :: No) , .. } , _ ,) ,] if second_delim == & Delimiter :: Parenthesis && name . as_str () == "a" => { } _ => panic ! ("value 4: {:?} {:?}" , second_delim , second_tts) , } } _ => panic ! ("value 2: {:?} {:?}" , macro_delim , macro_tts) , } } _ => panic ! ("value: {:?}" , tts) , } }) }
/* FP:tests.rs-1338 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0081
/* FP:tests.rs-1339 */ # [test] fn string_to_tts_1 () { create_default_session_globals_then (| | { let tts = string_to_stream ("fn a(b: i32) { b; }" . to_string ()) ; let expected = TokenStream :: new (vec ! [TokenTree :: token_alone (token :: Ident (kw :: Fn , IdentIsRaw :: No) , sp (0 , 2)) , TokenTree :: token_joint_hidden (token :: Ident (Symbol :: intern ("a") , IdentIsRaw :: No) , sp (3 , 4) ,) , TokenTree :: Delimited (DelimSpan :: from_pair (sp (4 , 5) , sp (11 , 12)) , DelimSpacing :: new (Spacing :: JointHidden , Spacing :: Alone) , Delimiter :: Parenthesis , TokenStream :: new (vec ! [TokenTree :: token_joint (token :: Ident (Symbol :: intern ("b") , IdentIsRaw :: No) , sp (5 , 6) ,) , TokenTree :: token_alone (token :: Colon , sp (6 , 7)) , TokenTree :: token_joint_hidden (token :: Ident (sym :: i32 , IdentIsRaw :: No) , sp (8 , 11) ,) ,]) ,) , TokenTree :: Delimited (DelimSpan :: from_pair (sp (13 , 14) , sp (18 , 19)) , DelimSpacing :: new (Spacing :: Alone , Spacing :: Alone) , Delimiter :: Brace , TokenStream :: new (vec ! [TokenTree :: token_joint (token :: Ident (Symbol :: intern ("b") , IdentIsRaw :: No) , sp (15 , 16) ,) , TokenTree :: token_alone (token :: Semi , sp (16 , 17)) ,]) ,) ,]) ; assert_eq ! (tts , expected) ; }) }
/* FP:tests.rs-1340 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0082
/* FP:tests.rs-1341 */ # [test] fn parse_use () { create_default_session_globals_then (| | { let use_s = "use foo::bar::baz;" ; let vitem = string_to_item (use_s . to_string ()) . unwrap () ; let vitem_s = item_to_string (& vitem) ; assert_eq ! (& vitem_s [..] , use_s) ; let use_s = "use foo::bar as baz;" ; let vitem = string_to_item (use_s . to_string ()) . unwrap () ; let vitem_s = item_to_string (& vitem) ; assert_eq ! (& vitem_s [..] , use_s) ; }) }
/* FP:tests.rs-1342 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0083
/* FP:tests.rs-1343 */ # [test] fn parse_extern_crate () { create_default_session_globals_then (| | { let ex_s = "extern crate foo;" ; let vitem = string_to_item (ex_s . to_string ()) . unwrap () ; let vitem_s = item_to_string (& vitem) ; assert_eq ! (& vitem_s [..] , ex_s) ; let ex_s = "extern crate foo as bar;" ; let vitem = string_to_item (ex_s . to_string ()) . unwrap () ; let vitem_s = item_to_string (& vitem) ; assert_eq ! (& vitem_s [..] , ex_s) ; }) }
/* FP:tests.rs-1344 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0084
/* FP:tests.rs-1345 */ fn get_spans_of_pat_idents (src : & str) -> Vec < Span > { let item = string_to_item (src . to_string ()) . unwrap () ; struct PatIdentVisitor { spans : Vec < Span > , } impl < 'a > visit :: Visitor < 'a > for PatIdentVisitor { fn visit_pat (& mut self , p : & 'a ast :: Pat) { match & p . kind { PatKind :: Ident (_ , ident , _) => { self . spans . push (ident . span) ; } _ => { visit :: walk_pat (self , p) ; } } } } let mut v = PatIdentVisitor { spans : Vec :: new () } ; visit :: walk_item (& mut v , & item) ; return v . spans ; }
/* FP:tests.rs-1346 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0085
/* FP:tests.rs-1347 */ # [test] fn span_of_self_arg_pat_idents_are_correct () { create_default_session_globals_then (| | { let srcs = ["impl z { fn a (&self, &myarg: i32) {} }" , "impl z { fn a (&mut self, &myarg: i32) {} }" , "impl z { fn a (&'a self, &myarg: i32) {} }" , "impl z { fn a (self, &myarg: i32) {} }" , "impl z { fn a (self: Foo, &myarg: i32) {} }" ,] ; for src in srcs { let spans = get_spans_of_pat_idents (src) ; let (lo , hi) = (spans [0] . lo () , spans [0] . hi ()) ; assert ! ("self" == & src [lo . to_usize () .. hi . to_usize ()] , "\"{}\" != \"self\". src=\"{}\"" , & src [lo . to_usize () .. hi . to_usize ()] , src) } }) }
/* FP:tests.rs-1348 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0086
/* FP:tests.rs-1349 */ # [test] fn parse_exprs () { create_default_session_globals_then (| | { string_to_expr ("3 + 4" . to_string ()) ; string_to_expr ("a::z.froob(b,&(987+3))" . to_string ()) ; }) }
/* FP:tests.rs-1350 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0087
/* FP:tests.rs-1351 */ # [test] fn attrs_fix_bug () { create_default_session_globals_then (| | { string_to_item ("pub fn mk_file_writer(path: &Path, flags: &[FileFlag])
/* FP:tests.rs-1352 */                 -> Result<Box<Writer>, String> {
/* FP:tests.rs-1353 */ #[cfg(windows)]
/* FP:tests.rs-1354 */ fn wb() -> c_int {
/* FP:tests.rs-1355 */     (O_WRONLY | libc::consts::os::extra::O_BINARY) as c_int
/* FP:tests.rs-1356 */ }
/* FP:tests.rs-1357 */ 
/* FP:tests.rs-1358 */ #[cfg(unix)]
/* FP:tests.rs-1359 */ fn wb() -> c_int { O_WRONLY as c_int }
/* FP:tests.rs-1360 */ 
/* FP:tests.rs-1361 */ let mut fflags: c_int = wb();
/* FP:tests.rs-1362 */ }" . to_string () ,) ; }) }
/* FP:tests.rs-1363 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0088
/* FP:tests.rs-1364 */ # [test] fn crlf_doc_comments () { create_default_session_globals_then (| | { let psess = psess () ; let name_1 = FileName :: Custom ("crlf_source_1" . to_string ()) ; let source = "/// doc comment\r\nfn foo() {}" . to_string () ; let item = parse_item_from_source_str (name_1 , source , & psess) . unwrap () . unwrap () ; let doc = item . attrs . iter () . filter_map (| at | at . doc_str ()) . next () . unwrap () ; assert_eq ! (doc . as_str () , " doc comment") ; let name_2 = FileName :: Custom ("crlf_source_2" . to_string ()) ; let source = "/// doc comment\r\n/// line 2\r\nfn foo() {}" . to_string () ; let item = parse_item_from_source_str (name_2 , source , & psess) . unwrap () . unwrap () ; let docs = item . attrs . iter () . filter_map (| at | at . doc_str ()) . collect :: < Vec < _ > > () ; let b : & [_] = & [Symbol :: intern (" doc comment") , Symbol :: intern (" line 2")] ; assert_eq ! (& docs [..] , b) ; let name_3 = FileName :: Custom ("clrf_source_3" . to_string ()) ; let source = "/** doc comment\r\n *  with CRLF */\r\nfn foo() {}" . to_string () ; let item = parse_item_from_source_str (name_3 , source , & psess) . unwrap () . unwrap () ; let doc = item . attrs . iter () . filter_map (| at | at . doc_str ()) . next () . unwrap () ; assert_eq ! (doc . as_str () , " doc comment\n *  with CRLF ") ; }) ; }
/* FP:tests.rs-1365 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0089
/* FP:tests.rs-1366 */ # [test] fn ttdelim_span () { fn parse_expr_from_source_str (name : FileName , source : String , psess : & ParseSess ,) -> PResult < '_ , Box < ast :: Expr > > { unwrap_or_emit_fatal (new_parser_from_source_str (psess , name , source , StripTokens :: Nothing)) . parse_expr () } create_default_session_globals_then (| | { let psess = psess () ; let expr = parse_expr_from_source_str (PathBuf :: from ("foo") . into () , "foo!( fn main() { body } )" . to_string () , & psess ,) . unwrap () ; let ast :: ExprKind :: MacCall (mac) = & expr . kind else { panic ! ("not a macro") } ; let span = mac . args . tokens . iter () . last () . unwrap () . span () ; match psess . source_map () . span_to_snippet (span) { Ok (s) => assert_eq ! (& s [..] , "{ body }") , Err (_) => panic ! ("could not get snippet") , } }) ; }
/* FP:tests.rs-1367 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0090
/* FP:tests.rs-1368 */ # [track_caller] fn look (p : & Parser < '_ > , dist : usize , kind : crate :: rustc_ast :: token :: TokenKind) { let tok = p . look_ahead (dist , | tok | * tok) ; assert_eq ! (kind , tok . kind) ; }
/* FP:tests.rs-1369 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0091
/* FP:tests.rs-1370 */ # [test] fn look_ahead () { create_default_session_globals_then (| | { let sym_f = Symbol :: intern ("f") ; let sym_x = Symbol :: intern ("x") ; # [allow (non_snake_case)] let sym_S = Symbol :: intern ("S") ; let raw_no = IdentIsRaw :: No ; let psess = psess () ; let mut p = string_to_parser (& psess , "fn f(x: u32) { x } struct S;" . to_string ()) ; look (& p , 0 , token :: Ident (kw :: Fn , raw_no)) ; look (& p , 1 , token :: Ident (sym_f , raw_no)) ; look (& p , 2 , token :: OpenParen) ; look (& p , 3 , token :: Ident (sym_x , raw_no)) ; look (& p , 4 , token :: Colon) ; look (& p , 5 , token :: Ident (sym :: u32 , raw_no)) ; look (& p , 6 , token :: CloseParen) ; look (& p , 7 , token :: OpenBrace) ; look (& p , 8 , token :: Ident (sym_x , raw_no)) ; look (& p , 9 , token :: CloseBrace) ; look (& p , 10 , token :: Ident (kw :: Struct , raw_no)) ; look (& p , 11 , token :: Ident (sym_S , raw_no)) ; look (& p , 12 , token :: Semi) ; look (& p , 13 , token :: Eof) ; look (& p , 14 , token :: Eof) ; look (& p , 15 , token :: Eof) ; look (& p , 100 , token :: Eof) ; for _ in 0 .. 3 { p . bump () ; } look (& p , 0 , token :: Ident (sym_x , raw_no)) ; look (& p , 1 , token :: Colon) ; look (& p , 2 , token :: Ident (sym :: u32 , raw_no)) ; look (& p , 3 , token :: CloseParen) ; look (& p , 4 , token :: OpenBrace) ; look (& p , 5 , token :: Ident (sym_x , raw_no)) ; look (& p , 6 , token :: CloseBrace) ; look (& p , 7 , token :: Ident (kw :: Struct , raw_no)) ; look (& p , 8 , token :: Ident (sym_S , raw_no)) ; look (& p , 9 , token :: Semi) ; look (& p , 10 , token :: Eof) ; look (& p , 11 , token :: Eof) ; look (& p , 100 , token :: Eof) ; for _ in 0 .. 9 { p . bump () ; } look (& p , 0 , token :: Semi) ; look (& p , 1 , token :: Eof) ; look (& p , 100 , token :: Eof) ; p . bump () ; look (& p , 0 , token :: Eof) ; look (& p , 1 , token :: Eof) ; look (& p , 100 , token :: Eof) ; p . bump () ; look (& p , 0 , token :: Eof) ; look (& p , 1 , token :: Eof) ; look (& p , 100 , token :: Eof) ; }) ; }
/* FP:tests.rs-1371 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0092
/* FP:tests.rs-1373 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0093
/* FP:tests.rs-1374 */ # [test] fn debug_lookahead () { create_default_session_globals_then (| | { let psess = psess () ; let mut p = string_to_parser (& psess , "fn f(x: u32) { x } struct S;" . to_string ()) ; assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (0)) , "Parser {
/* FP:tests.rs-1375 */     prev_token: Token {
/* FP:tests.rs-1376 */         kind: Question,
/* FP:tests.rs-1377 */         span: Span {
/* FP:tests.rs-1378 */             lo: BytePos(
/* FP:tests.rs-1379 */                 0,
/* FP:tests.rs-1380 */             ),
/* FP:tests.rs-1381 */             hi: BytePos(
/* FP:tests.rs-1382 */                 0,
/* FP:tests.rs-1383 */             ),
/* FP:tests.rs-1384 */             ctxt: #0,
/* FP:tests.rs-1385 */         },
/* FP:tests.rs-1386 */     },
/* FP:tests.rs-1387 */     tokens: [],
/* FP:tests.rs-1388 */     approx_token_stream_pos: 0,
/* FP:tests.rs-1389 */     ..
/* FP:tests.rs-1390 */ }") ; assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (7)) , "Parser {
/* FP:tests.rs-1391 */     prev_token: Token {
/* FP:tests.rs-1392 */         kind: Question,
/* FP:tests.rs-1393 */         span: Span {
/* FP:tests.rs-1394 */             lo: BytePos(
/* FP:tests.rs-1395 */                 0,
/* FP:tests.rs-1396 */             ),
/* FP:tests.rs-1397 */             hi: BytePos(
/* FP:tests.rs-1398 */                 0,
/* FP:tests.rs-1399 */             ),
/* FP:tests.rs-1400 */             ctxt: #0,
/* FP:tests.rs-1401 */         },
/* FP:tests.rs-1402 */     },
/* FP:tests.rs-1403 */     tokens: [
/* FP:tests.rs-1404 */         Ident(
/* FP:tests.rs-1405 */             \"fn\",
/* FP:tests.rs-1406 */             No,
/* FP:tests.rs-1407 */         ),
/* FP:tests.rs-1408 */         Ident(
/* FP:tests.rs-1409 */             \"f\",
/* FP:tests.rs-1410 */             No,
/* FP:tests.rs-1411 */         ),
/* FP:tests.rs-1412 */         OpenParen,
/* FP:tests.rs-1413 */         Ident(
/* FP:tests.rs-1414 */             \"x\",
/* FP:tests.rs-1415 */             No,
/* FP:tests.rs-1416 */         ),
/* FP:tests.rs-1417 */         Colon,
/* FP:tests.rs-1418 */         Ident(
/* FP:tests.rs-1419 */             \"u32\",
/* FP:tests.rs-1420 */             No,
/* FP:tests.rs-1421 */         ),
/* FP:tests.rs-1422 */         CloseParen,
/* FP:tests.rs-1423 */     ],
/* FP:tests.rs-1424 */     approx_token_stream_pos: 0,
/* FP:tests.rs-1425 */     ..
/* FP:tests.rs-1426 */ }") ; assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (15)) , "Parser {
/* FP:tests.rs-1427 */     prev_token: Token {
/* FP:tests.rs-1428 */         kind: Question,
/* FP:tests.rs-1429 */         span: Span {
/* FP:tests.rs-1430 */             lo: BytePos(
/* FP:tests.rs-1431 */                 0,
/* FP:tests.rs-1432 */             ),
/* FP:tests.rs-1433 */             hi: BytePos(
/* FP:tests.rs-1434 */                 0,
/* FP:tests.rs-1435 */             ),
/* FP:tests.rs-1436 */             ctxt: #0,
/* FP:tests.rs-1437 */         },
/* FP:tests.rs-1438 */     },
/* FP:tests.rs-1439 */     tokens: [
/* FP:tests.rs-1440 */         Ident(
/* FP:tests.rs-1441 */             \"fn\",
/* FP:tests.rs-1442 */             No,
/* FP:tests.rs-1443 */         ),
/* FP:tests.rs-1444 */         Ident(
/* FP:tests.rs-1445 */             \"f\",
/* FP:tests.rs-1446 */             No,
/* FP:tests.rs-1447 */         ),
/* FP:tests.rs-1448 */         OpenParen,
/* FP:tests.rs-1449 */         Ident(
/* FP:tests.rs-1450 */             \"x\",
/* FP:tests.rs-1451 */             No,
/* FP:tests.rs-1452 */         ),
/* FP:tests.rs-1453 */         Colon,
/* FP:tests.rs-1454 */         Ident(
/* FP:tests.rs-1455 */             \"u32\",
/* FP:tests.rs-1456 */             No,
/* FP:tests.rs-1457 */         ),
/* FP:tests.rs-1458 */         CloseParen,
/* FP:tests.rs-1459 */         OpenBrace,
/* FP:tests.rs-1460 */         Ident(
/* FP:tests.rs-1461 */             \"x\",
/* FP:tests.rs-1462 */             No,
/* FP:tests.rs-1463 */         ),
/* FP:tests.rs-1464 */         CloseBrace,
/* FP:tests.rs-1465 */         Ident(
/* FP:tests.rs-1466 */             \"struct\",
/* FP:tests.rs-1467 */             No,
/* FP:tests.rs-1468 */         ),
/* FP:tests.rs-1469 */         Ident(
/* FP:tests.rs-1470 */             \"S\",
/* FP:tests.rs-1471 */             No,
/* FP:tests.rs-1472 */         ),
/* FP:tests.rs-1473 */         Semi,
/* FP:tests.rs-1474 */         Eof,
/* FP:tests.rs-1475 */     ],
/* FP:tests.rs-1476 */     approx_token_stream_pos: 0,
/* FP:tests.rs-1477 */     ..
/* FP:tests.rs-1478 */ }") ; for _ in 0 .. 8 { p . bump () ; } assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (1)) , "Parser {
/* FP:tests.rs-1479 */     prev_token: Token {
/* FP:tests.rs-1480 */         kind: OpenBrace,
/* FP:tests.rs-1481 */         span: Span {
/* FP:tests.rs-1482 */             lo: BytePos(
/* FP:tests.rs-1483 */                 13,
/* FP:tests.rs-1484 */             ),
/* FP:tests.rs-1485 */             hi: BytePos(
/* FP:tests.rs-1486 */                 14,
/* FP:tests.rs-1487 */             ),
/* FP:tests.rs-1488 */             ctxt: #0,
/* FP:tests.rs-1489 */         },
/* FP:tests.rs-1490 */     },
/* FP:tests.rs-1491 */     tokens: [
/* FP:tests.rs-1492 */         Ident(
/* FP:tests.rs-1493 */             \"x\",
/* FP:tests.rs-1494 */             No,
/* FP:tests.rs-1495 */         ),
/* FP:tests.rs-1496 */     ],
/* FP:tests.rs-1497 */     approx_token_stream_pos: 8,
/* FP:tests.rs-1498 */     ..
/* FP:tests.rs-1499 */ }") ; assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (4)) , "Parser {
/* FP:tests.rs-1500 */     prev_token: Token {
/* FP:tests.rs-1501 */         kind: OpenBrace,
/* FP:tests.rs-1502 */         span: Span {
/* FP:tests.rs-1503 */             lo: BytePos(
/* FP:tests.rs-1504 */                 13,
/* FP:tests.rs-1505 */             ),
/* FP:tests.rs-1506 */             hi: BytePos(
/* FP:tests.rs-1507 */                 14,
/* FP:tests.rs-1508 */             ),
/* FP:tests.rs-1509 */             ctxt: #0,
/* FP:tests.rs-1510 */         },
/* FP:tests.rs-1511 */     },
/* FP:tests.rs-1512 */     tokens: [
/* FP:tests.rs-1513 */         Ident(
/* FP:tests.rs-1514 */             \"x\",
/* FP:tests.rs-1515 */             No,
/* FP:tests.rs-1516 */         ),
/* FP:tests.rs-1517 */         CloseBrace,
/* FP:tests.rs-1518 */         Ident(
/* FP:tests.rs-1519 */             \"struct\",
/* FP:tests.rs-1520 */             No,
/* FP:tests.rs-1521 */         ),
/* FP:tests.rs-1522 */         Ident(
/* FP:tests.rs-1523 */             \"S\",
/* FP:tests.rs-1524 */             No,
/* FP:tests.rs-1525 */         ),
/* FP:tests.rs-1526 */     ],
/* FP:tests.rs-1527 */     approx_token_stream_pos: 8,
/* FP:tests.rs-1528 */     ..
/* FP:tests.rs-1529 */ }") ; for _ in 0 .. 6 { p . bump () ; } assert_eq ! (& format ! ("{:#?}" , p . debug_lookahead (3)) , "Parser {
/* FP:tests.rs-1530 */     prev_token: Token {
/* FP:tests.rs-1531 */         kind: Eof,
/* FP:tests.rs-1532 */         span: Span {
/* FP:tests.rs-1533 */             lo: BytePos(
/* FP:tests.rs-1534 */                 27,
/* FP:tests.rs-1535 */             ),
/* FP:tests.rs-1536 */             hi: BytePos(
/* FP:tests.rs-1537 */                 28,
/* FP:tests.rs-1538 */             ),
/* FP:tests.rs-1539 */             ctxt: #0,
/* FP:tests.rs-1540 */         },
/* FP:tests.rs-1541 */     },
/* FP:tests.rs-1542 */     tokens: [
/* FP:tests.rs-1543 */         Eof,
/* FP:tests.rs-1544 */     ],
/* FP:tests.rs-1545 */     approx_token_stream_pos: 14,
/* FP:tests.rs-1546 */     ..
/* FP:tests.rs-1547 */ }") ; }) ; }
/* FP:tests.rs-1548 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0094
/* FP:tests.rs-1550 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0095
/* FP:tests.rs-1551 */ # [test] fn eqmodws () { assert_eq ! (matches_codepattern ("" , "") , true) ; assert_eq ! (matches_codepattern ("" , "a") , false) ; assert_eq ! (matches_codepattern ("a" , "") , false) ; assert_eq ! (matches_codepattern ("a" , "a") , true) ; assert_eq ! (matches_codepattern ("a b" , "a   \n\t\r  b") , true) ; assert_eq ! (matches_codepattern ("a b " , "a   \n\t\r  b") , true) ; assert_eq ! (matches_codepattern ("a b" , "a   \n\t\r  b ") , false) ; assert_eq ! (matches_codepattern ("a   b" , "a b") , true) ; assert_eq ! (matches_codepattern ("ab" , "a b") , false) ; assert_eq ! (matches_codepattern ("a   b" , "ab") , true) ; assert_eq ! (matches_codepattern (" a   b" , "ab") , true) ; }
/* FP:tests.rs-1552 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0096
/* FP:tests.rs-1553 */ # [test] fn pattern_whitespace () { assert_eq ! (matches_codepattern ("" , "\x0C") , false) ; assert_eq ! (matches_codepattern ("a b " , "a   \u{0085}\n\t\r  b") , true) ; assert_eq ! (matches_codepattern ("a b" , "a   \u{0085}\n\t\r  b ") , false) ; }
/* FP:tests.rs-1554 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_tests_FN_0097
/* FP:tests.rs-1555 */ # [test] fn non_pattern_whitespace () { assert_eq ! (matches_codepattern ("a b" , "a\u{2002}b") , false) ; assert_eq ! (matches_codepattern ("a   b" , "a\u{2002}b") , false) ; assert_eq ! (matches_codepattern ("\u{205F}a   b" , "ab") , false) ; assert_eq ! (matches_codepattern ("a  \u{3000}b" , "ab") , false) ; }