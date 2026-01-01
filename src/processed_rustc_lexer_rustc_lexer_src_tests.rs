/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_USE_0001
/* FP:tests.rs-0002 */ use expect_test :: { Expect , expect } ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_USE_0002
/* FP:tests.rs-0004 */ use super :: * ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0003
/* FP:tests.rs-0006 */ fn check_raw_str (s : & str , expected : Result < u8 , RawStrError >) { let s = & format ! ("r{}" , s) ; let mut cursor = Cursor :: new (s , FrontmatterAllowed :: No) ; cursor . bump () ; let res = cursor . raw_double_quoted_string (0) ; assert_eq ! (res , expected) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn test_naked_raw_str () { check_raw_str (r#""abc""# , Ok (0)) ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn test_raw_no_start () { check_raw_str (r##""abc"#"## , Ok (0)) ; }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn test_too_many_terminators () { check_raw_str (r###"#"abc"##"### , Ok (1)) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn test_unterminated () { check_raw_str (r#"#"abc"# , Err (RawStrError :: NoTerminator { expected : 1 , found : 0 , possible_terminator_offset : None }) ,) ; check_raw_str (r###"##"abc"#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 1 , possible_terminator_offset : Some (7) , }) ,) ; check_raw_str (r###"##"abc#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 0 , possible_terminator_offset : None }) ,) }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn test_invalid_start () { check_raw_str (r##"#~"abc"#"## , Err (RawStrError :: InvalidStarter { bad_char : '~' })) ; }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0009
/* FP:tests.rs-0018 */ # [test] fn test_unterminated_no_pound () { check_raw_str (r#"""# , Err (RawStrError :: NoTerminator { expected : 0 , found : 0 , possible_terminator_offset : None }) ,) ; }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0010
/* FP:tests.rs-0020 */ # [test] fn test_too_many_hashes () { let max_count = u8 :: MAX ; let hashes1 = "#" . repeat (max_count as usize) ; let hashes2 = "#" . repeat (max_count as usize + 1) ; let middle = "\"abc\"" ; let s1 = [& hashes1 , middle , & hashes1] . join ("") ; let s2 = [& hashes2 , middle , & hashes2] . join ("") ; check_raw_str (& s1 , Ok (255)) ; check_raw_str (& s2 , Err (RawStrError :: TooManyDelimiters { found : u32 :: from (max_count) + 1 })) ; }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0011
/* FP:tests.rs-0022 */ # [test] fn test_valid_shebang () { let input = "#!/bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#!    /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#!    [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! /* blah */  /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#! /* blah */  [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! // blah\n/bin/bash" ; assert_eq ! (strip_shebang (input) , Some (10)) ; let input = "#! // blah\n[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! /* blah\nblah\nblah */  /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (10)) ; let input = "#! /* blah\nblah\nblah */  [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#!\n/bin/sh" ; assert_eq ! (strip_shebang (input) , Some (2)) ; let input = "#!\n[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "\n#!/bin/bash" ; assert_eq ! (strip_shebang (input) , None) ; let input = "\n#[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0012
/* FP:tests.rs-0024 */ fn check_lexing (src : & str , frontmatter_allowed : FrontmatterAllowed , expect : Expect) { let actual : String = tokenize (src , frontmatter_allowed) . map (| token | format ! ("{:?}\n" , token)) . collect () ; expect . assert_eq (& actual) }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn smoke_test () { check_lexing ("/* my source file */ fn main() { println!(\"zebra\"); }\n" , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0027 */             Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
/* FP:tests.rs-0028 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0029 */             Token { kind: Ident, len: 2 }
/* FP:tests.rs-0030 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0031 */             Token { kind: Ident, len: 4 }
/* FP:tests.rs-0032 */             Token { kind: OpenParen, len: 1 }
/* FP:tests.rs-0033 */             Token { kind: CloseParen, len: 1 }
/* FP:tests.rs-0034 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0035 */             Token { kind: OpenBrace, len: 1 }
/* FP:tests.rs-0036 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0037 */             Token { kind: Ident, len: 7 }
/* FP:tests.rs-0038 */             Token { kind: Bang, len: 1 }
/* FP:tests.rs-0039 */             Token { kind: OpenParen, len: 1 }
/* FP:tests.rs-0040 */             Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
/* FP:tests.rs-0041 */             Token { kind: CloseParen, len: 1 }
/* FP:tests.rs-0042 */             Token { kind: Semi, len: 1 }
/* FP:tests.rs-0043 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0044 */             Token { kind: CloseBrace, len: 1 }
/* FP:tests.rs-0045 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0046 */         "#]] ,) }
/* FP:tests.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0014
/* FP:tests.rs-0048 */ # [test] fn comment_flavors () { check_lexing (r"
/* FP:tests.rs-0049 */ // line
/* FP:tests.rs-0050 */ //// line as well
/* FP:tests.rs-0051 */ /// outer doc line
/* FP:tests.rs-0052 */ // inner doc line
/* FP:tests.rs-0053 */ /* block */
/* FP:tests.rs-0054 */ /**/
/* FP:tests.rs-0055 */ /*** also block */
/* FP:tests.rs-0056 */ /** outer doc block */
/* FP:tests.rs-0057 */ /* inner doc block */
/* FP:tests.rs-0058 */ " , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0059 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0060 */             Token { kind: LineComment { doc_style: None }, len: 7 }
/* FP:tests.rs-0061 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0062 */             Token { kind: LineComment { doc_style: None }, len: 17 }
/* FP:tests.rs-0063 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0064 */             Token { kind: LineComment { doc_style: Some(Outer) }, len: 18 }
/* FP:tests.rs-0065 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0066 */             Token { kind: LineComment { doc_style: Some(Inner) }, len: 18 }
/* FP:tests.rs-0067 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0068 */             Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
/* FP:tests.rs-0069 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0070 */             Token { kind: BlockComment { doc_style: None, terminated: true }, len: 4 }
/* FP:tests.rs-0071 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0072 */             Token { kind: BlockComment { doc_style: None, terminated: true }, len: 18 }
/* FP:tests.rs-0073 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0074 */             Token { kind: BlockComment { doc_style: Some(Outer), terminated: true }, len: 22 }
/* FP:tests.rs-0075 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0076 */             Token { kind: BlockComment { doc_style: Some(Inner), terminated: true }, len: 22 }
/* FP:tests.rs-0077 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0078 */         "#]] ,) }
/* FP:tests.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0015
/* FP:tests.rs-0080 */ # [test] fn nested_block_comments () { check_lexing ("/* /* */ */'a'" , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0081 */             Token { kind: BlockComment { doc_style: None, terminated: true }, len: 11 }
/* FP:tests.rs-0082 */             Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0083 */         "#]] ,) }
/* FP:tests.rs-0084 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0016
/* FP:tests.rs-0085 */ # [test] fn characters () { check_lexing ("'a' ' ' '\\n'" , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0086 */             Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0087 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0088 */             Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0089 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0090 */             Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 4 }, len: 4 }
/* FP:tests.rs-0091 */         "#]] ,) ; }
/* FP:tests.rs-0092 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0017
/* FP:tests.rs-0093 */ # [test] fn lifetime () { check_lexing ("'abc" , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0094 */             Token { kind: Lifetime { starts_with_number: false }, len: 4 }
/* FP:tests.rs-0095 */         "#]] ,) ; }
/* FP:tests.rs-0096 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0018
/* FP:tests.rs-0097 */ # [test] fn raw_string () { check_lexing ("r###\"\"#a\\b\x00c\"\"###" , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0098 */             Token { kind: Literal { kind: RawStr { n_hashes: Some(3) }, suffix_start: 17 }, len: 17 }
/* FP:tests.rs-0099 */         "#]] ,) }
/* FP:tests.rs-0100 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0019
/* FP:tests.rs-0101 */ # [test] fn literal_suffixes () { check_lexing (r####"
/* FP:tests.rs-0102 */ 'a'
/* FP:tests.rs-0103 */ b'a'
/* FP:tests.rs-0104 */ "a"
/* FP:tests.rs-0105 */ b"a"
/* FP:tests.rs-0106 */ 1234
/* FP:tests.rs-0107 */ 0b101
/* FP:tests.rs-0108 */ 0xABC
/* FP:tests.rs-0109 */ 1.0
/* FP:tests.rs-0110 */ 1.0e10
/* FP:tests.rs-0111 */ 2us
/* FP:tests.rs-0112 */ r###"raw"###suffix
/* FP:tests.rs-0113 */ br###"raw"###suffix
/* FP:tests.rs-0114 */ "#### , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0115 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0116 */             Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0117 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0118 */             Token { kind: Literal { kind: Byte { terminated: true }, suffix_start: 4 }, len: 4 }
/* FP:tests.rs-0119 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0120 */             Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0121 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0122 */             Token { kind: Literal { kind: ByteStr { terminated: true }, suffix_start: 4 }, len: 4 }
/* FP:tests.rs-0123 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0124 */             Token { kind: Literal { kind: Int { base: Decimal, empty_int: false }, suffix_start: 4 }, len: 4 }
/* FP:tests.rs-0125 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0126 */             Token { kind: Literal { kind: Int { base: Binary, empty_int: false }, suffix_start: 5 }, len: 5 }
/* FP:tests.rs-0127 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0128 */             Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false }, suffix_start: 5 }, len: 5 }
/* FP:tests.rs-0129 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0130 */             Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: false }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0131 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0132 */             Token { kind: Literal { kind: Float { base: Decimal, empty_exponent: false }, suffix_start: 6 }, len: 6 }
/* FP:tests.rs-0133 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0134 */             Token { kind: Literal { kind: Int { base: Decimal, empty_int: false }, suffix_start: 1 }, len: 3 }
/* FP:tests.rs-0135 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0136 */             Token { kind: Literal { kind: RawStr { n_hashes: Some(3) }, suffix_start: 12 }, len: 18 }
/* FP:tests.rs-0137 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0138 */             Token { kind: Literal { kind: RawByteStr { n_hashes: Some(3) }, suffix_start: 13 }, len: 19 }
/* FP:tests.rs-0139 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0140 */         "#]] ,) }
/* FP:tests.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0020
/* FP:tests.rs-0142 */ # [test] fn frontmatter_allowed () { check_lexing (r#"
/* FP:tests.rs-0143 */ ---cargo
/* FP:tests.rs-0144 */ [dependencies]
/* FP:tests.rs-0145 */ clap = "4"
/* FP:tests.rs-0146 */ ---
/* FP:tests.rs-0147 */ 
/* FP:tests.rs-0148 */ fn main() {}
/* FP:tests.rs-0149 */ "# , FrontmatterAllowed :: Yes , expect ! [[r#"
/* FP:tests.rs-0150 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0151 */             Token { kind: Frontmatter { has_invalid_preceding_whitespace: false, invalid_infostring: false }, len: 38 }
/* FP:tests.rs-0152 */             Token { kind: Whitespace, len: 2 }
/* FP:tests.rs-0153 */             Token { kind: Ident, len: 2 }
/* FP:tests.rs-0154 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0155 */             Token { kind: Ident, len: 4 }
/* FP:tests.rs-0156 */             Token { kind: OpenParen, len: 1 }
/* FP:tests.rs-0157 */             Token { kind: CloseParen, len: 1 }
/* FP:tests.rs-0158 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0159 */             Token { kind: OpenBrace, len: 1 }
/* FP:tests.rs-0160 */             Token { kind: CloseBrace, len: 1 }
/* FP:tests.rs-0161 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0162 */         "#]] ,) }
/* FP:tests.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lexer_src_tests_FN_0021
/* FP:tests.rs-0164 */ # [test] fn frontmatter_disallowed () { check_lexing (r#"
/* FP:tests.rs-0165 */ ---cargo
/* FP:tests.rs-0166 */ [dependencies]
/* FP:tests.rs-0167 */ clap = "4"
/* FP:tests.rs-0168 */ ---
/* FP:tests.rs-0169 */ 
/* FP:tests.rs-0170 */ fn main() {}
/* FP:tests.rs-0171 */ "# , FrontmatterAllowed :: No , expect ! [[r#"
/* FP:tests.rs-0172 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0173 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0174 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0175 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0176 */             Token { kind: Ident, len: 5 }
/* FP:tests.rs-0177 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0178 */             Token { kind: OpenBracket, len: 1 }
/* FP:tests.rs-0179 */             Token { kind: Ident, len: 12 }
/* FP:tests.rs-0180 */             Token { kind: CloseBracket, len: 1 }
/* FP:tests.rs-0181 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0182 */             Token { kind: Ident, len: 4 }
/* FP:tests.rs-0183 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0184 */             Token { kind: Eq, len: 1 }
/* FP:tests.rs-0185 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0186 */             Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
/* FP:tests.rs-0187 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0188 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0189 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0190 */             Token { kind: Minus, len: 1 }
/* FP:tests.rs-0191 */             Token { kind: Whitespace, len: 2 }
/* FP:tests.rs-0192 */             Token { kind: Ident, len: 2 }
/* FP:tests.rs-0193 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0194 */             Token { kind: Ident, len: 4 }
/* FP:tests.rs-0195 */             Token { kind: OpenParen, len: 1 }
/* FP:tests.rs-0196 */             Token { kind: CloseParen, len: 1 }
/* FP:tests.rs-0197 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0198 */             Token { kind: OpenBrace, len: 1 }
/* FP:tests.rs-0199 */             Token { kind: CloseBrace, len: 1 }
/* FP:tests.rs-0200 */             Token { kind: Whitespace, len: 1 }
/* FP:tests.rs-0201 */         "#]] ,) }