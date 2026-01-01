/* FP:parse.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_USE_0001
/* FP:parse.rs-0002 */ use ParseOpt as PO ;
/* FP:parse.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_USE_0002
/* FP:parse.rs-0004 */ use super :: * ;
/* FP:parse.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0003
/* FP:parse.rs-0006 */ # [test] fn test_parse_simple () { let buf = "**abcd** rest" ; let (t , r) = parse_simple_pat (buf . as_bytes () , b"**" , b"**" , PO :: None , MdTree :: Strong) . unwrap () ; assert_eq ! (t , MdTree :: Strong ("abcd")) ; assert_eq ! (r , b" rest") ; let buf = r"**abcd\** rest" ; let res = parse_simple_pat (buf . as_bytes () , b"**" , b"**" , PO :: None , MdTree :: Strong) ; assert ! (res . is_none ()) ; }
/* FP:parse.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0004
/* FP:parse.rs-0008 */ # [test] fn test_parse_comment () { let opt = PO :: TrimNoEsc ; let buf = "<!-- foobar! -->rest" ; let (t , r) = parse_simple_pat (buf . as_bytes () , CMT_S , CMT_E , opt , MdTree :: Comment) . unwrap () ; assert_eq ! (t , MdTree :: Comment ("foobar!")) ; assert_eq ! (r , b"rest") ; let buf = r"<!-- foobar! \-->rest" ; let (t , r) = parse_simple_pat (buf . as_bytes () , CMT_S , CMT_E , opt , MdTree :: Comment) . unwrap () ; assert_eq ! (t , MdTree :: Comment (r"foobar! \")) ; assert_eq ! (r , b"rest") ; }
/* FP:parse.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0005
/* FP:parse.rs-0010 */ # [test] fn test_parse_heading () { let buf1 = "# Top level\nrest" ; let (t , r) = parse_heading (buf1 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: Heading (1 , vec ! [MdTree :: PlainText ("Top level")] . into ())) ; assert_eq ! (r , b"\nrest") ; let buf1 = "# Empty" ; let (t , r) = parse_heading (buf1 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: Heading (1 , vec ! [MdTree :: PlainText ("Empty")] . into ())) ; assert_eq ! (r , b"") ; let buf2 = "### Top `level` _woo_\nrest" ; let (t , r) = parse_heading (buf2 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: Heading (3 , vec ! [MdTree :: PlainText ("Top ") , MdTree :: CodeInline ("level") , MdTree :: PlainText (" ") , MdTree :: Emphasis ("woo") ,] . into ())) ; assert_eq ! (r , b"\nrest") ; }
/* FP:parse.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0006
/* FP:parse.rs-0012 */ # [test] fn test_parse_code_inline () { let buf1 = "`abcd` rest" ; let (t , r) = parse_codeinline (buf1 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: CodeInline ("abcd")) ; assert_eq ! (r , b" rest") ; let buf2 = "```ab\ncd``` rest" ; let (t , r) = parse_codeinline (buf2 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: CodeInline ("ab\ncd")) ; assert_eq ! (r , b" rest") ; let buf3 = r"`abcd\` rest" ; let (t , r) = parse_codeinline (buf3 . as_bytes ()) . unwrap () ; assert_eq ! (t , MdTree :: CodeInline (r"abcd\")) ; assert_eq ! (r , b" rest") ; }
/* FP:parse.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0007
/* FP:parse.rs-0014 */ # [test] fn test_parse_code_block () { let buf1 = "```rust\ncode\ncode\n```\nleftovers" ; let (t , r) = parse_codeblock (buf1 . as_bytes ()) ; assert_eq ! (t , MdTree :: CodeBlock { txt : "code\ncode" , lang : Some ("rust") }) ; assert_eq ! (r , b"\nleftovers") ; let buf2 = "`````\ncode\ncode````\n`````\nleftovers" ; let (t , r) = parse_codeblock (buf2 . as_bytes ()) ; assert_eq ! (t , MdTree :: CodeBlock { txt : "code\ncode````" , lang : None }) ; assert_eq ! (r , b"\nleftovers") ; }
/* FP:parse.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0008
/* FP:parse.rs-0016 */ # [test] fn test_parse_link () { let simple = "[see here](docs.rs) other" ; let (t , r) = parse_any_link (simple . as_bytes () , false) . unwrap () ; assert_eq ! (t , MdTree :: Link { disp : "see here" , link : "docs.rs" }) ; assert_eq ! (r , b" other") ; let simple_toplevel = "[see here](docs.rs) other" ; let (t , r) = parse_any_link (simple_toplevel . as_bytes () , true) . unwrap () ; assert_eq ! (t , MdTree :: Link { disp : "see here" , link : "docs.rs" }) ; assert_eq ! (r , b" other") ; let reference = "[see here] other" ; let (t , r) = parse_any_link (reference . as_bytes () , true) . unwrap () ; assert_eq ! (t , MdTree :: RefLink { disp : "see here" , id : None }) ; assert_eq ! (r , b" other") ; let reference_full = "[see here][docs-rs] other" ; let (t , r) = parse_any_link (reference_full . as_bytes () , false) . unwrap () ; assert_eq ! (t , MdTree :: RefLink { disp : "see here" , id : Some ("docs-rs") }) ; assert_eq ! (r , b" other") ; let reference_def = "[see here]: docs.rs\nother" ; let (t , r) = parse_any_link (reference_def . as_bytes () , true) . unwrap () ; assert_eq ! (t , MdTree :: LinkDef { id : "see here" , link : "docs.rs" }) ; assert_eq ! (r , b"\nother") ; }
/* FP:parse.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0009
/* FP:parse.rs-0018 */ const IND1 : & str = r"test standard
/* FP:parse.rs-0019 */     ind
/* FP:parse.rs-0020 */     ind2
/* FP:parse.rs-0021 */ not ind" ;
/* FP:parse.rs-0022 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0010
/* FP:parse.rs-0023 */ const IND2 : & str = r"test end of stream
/* FP:parse.rs-0024 */   1
/* FP:parse.rs-0025 */   2
/* FP:parse.rs-0026 */ " ;
/* FP:parse.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0011
/* FP:parse.rs-0028 */ const IND3 : & str = r"test empty lines
/* FP:parse.rs-0029 */   1
/* FP:parse.rs-0030 */   2
/* FP:parse.rs-0031 */ 
/* FP:parse.rs-0032 */ not ind" ;
/* FP:parse.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0012
/* FP:parse.rs-0034 */ # [test] fn test_indented_section () { let (t , r) = get_indented_section (IND1 . as_bytes ()) ; assert_eq ! (str :: from_utf8 (t) . unwrap () , "test standard\n    ind\n    ind2") ; assert_eq ! (str :: from_utf8 (r) . unwrap () , "\nnot ind") ; let (txt , rest) = get_indented_section (IND2 . as_bytes ()) ; assert_eq ! (str :: from_utf8 (txt) . unwrap () , "test end of stream\n  1\n  2\n") ; assert_eq ! (str :: from_utf8 (rest) . unwrap () , "") ; let (txt , rest) = get_indented_section (IND3 . as_bytes ()) ; assert_eq ! (str :: from_utf8 (txt) . unwrap () , "test empty lines\n  1\n  2\n") ; assert_eq ! (str :: from_utf8 (rest) . unwrap () , "\nnot ind") ; }
/* FP:parse.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0013
/* FP:parse.rs-0036 */ const HBT : & str = r"# Heading
/* FP:parse.rs-0037 */ 
/* FP:parse.rs-0038 */ content" ;
/* FP:parse.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0014
/* FP:parse.rs-0040 */ # [test] fn test_heading_breaks () { let expected = vec ! [MdTree :: Heading (1 , vec ! [MdTree :: PlainText ("Heading")] . into ()) , MdTree :: PlainText ("content") ,] . into () ; let res = entrypoint (HBT) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0015
/* FP:parse.rs-0042 */ const NL1 : & str = r"start
/* FP:parse.rs-0043 */ 
/* FP:parse.rs-0044 */ end" ;
/* FP:parse.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0016
/* FP:parse.rs-0046 */ const NL2 : & str = r"start
/* FP:parse.rs-0047 */ 
/* FP:parse.rs-0048 */ 
/* FP:parse.rs-0049 */ end" ;
/* FP:parse.rs-0050 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0017
/* FP:parse.rs-0051 */ const NL3 : & str = r"start
/* FP:parse.rs-0052 */ 
/* FP:parse.rs-0053 */ 
/* FP:parse.rs-0054 */ 
/* FP:parse.rs-0055 */ end" ;
/* FP:parse.rs-0056 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0018
/* FP:parse.rs-0057 */ # [test] fn test_newline_breaks () { let expected = vec ! [MdTree :: PlainText ("start") , MdTree :: ParagraphBreak , MdTree :: PlainText ("end")] . into () ; for (idx , check) in [NL1 , NL2 , NL3] . iter () . enumerate () { let res = entrypoint (check) ; assert_eq ! (res , expected , "failed {idx}") ; } }
/* FP:parse.rs-0058 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0019
/* FP:parse.rs-0059 */ const WRAP : & str = "plain _italics
/* FP:parse.rs-0060 */ italics_" ;
/* FP:parse.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0020
/* FP:parse.rs-0062 */ # [test] fn test_wrap_pattern () { let expected = vec ! [MdTree :: PlainText ("plain ") , MdTree :: Emphasis ("italics") , MdTree :: Emphasis (" ") , MdTree :: Emphasis ("italics") ,] . into () ; let res = entrypoint (WRAP) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0021
/* FP:parse.rs-0064 */ const WRAP_NOTXT : & str = r"_italics_
/* FP:parse.rs-0065 */ **bold**" ;
/* FP:parse.rs-0066 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0022
/* FP:parse.rs-0067 */ # [test] fn test_wrap_notxt () { let expected = vec ! [MdTree :: Emphasis ("italics") , MdTree :: PlainText (" ") , MdTree :: Strong ("bold")] . into () ; let res = entrypoint (WRAP_NOTXT) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0068 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0023
/* FP:parse.rs-0069 */ const MIXED_LIST : & str = r"start
/* FP:parse.rs-0070 */ - _italics item_
/* FP:parse.rs-0071 */ <!-- comment -->
/* FP:parse.rs-0072 */ - **bold item**
/* FP:parse.rs-0073 */   second line [link1](foobar1)
/* FP:parse.rs-0074 */   third line [link2][link-foo]
/* FP:parse.rs-0075 */ -   :crab:
/* FP:parse.rs-0076 */     extra indent
/* FP:parse.rs-0077 */ end
/* FP:parse.rs-0078 */ [link-foo]: foobar2
/* FP:parse.rs-0079 */ " ;
/* FP:parse.rs-0080 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0024
/* FP:parse.rs-0081 */ # [test] fn test_list () { let expected = vec ! [MdTree :: PlainText ("start") , MdTree :: ParagraphBreak , MdTree :: UnorderedListItem (vec ! [MdTree :: Emphasis ("italics item")] . into ()) , MdTree :: LineBreak , MdTree :: UnorderedListItem (vec ! [MdTree :: Strong ("bold item") , MdTree :: PlainText (" second line ") , MdTree :: Link { disp : "link1" , link : "foobar1" } , MdTree :: PlainText (" third line ") , MdTree :: Link { disp : "link2" , link : "foobar2" } ,] . into () ,) , MdTree :: LineBreak , MdTree :: UnorderedListItem (vec ! [MdTree :: PlainText ("🦀") , MdTree :: PlainText (" extra indent")] . into () ,) , MdTree :: ParagraphBreak , MdTree :: PlainText ("end") ,] . into () ; let res = entrypoint (MIXED_LIST) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0082 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0025
/* FP:parse.rs-0083 */ const SMOOSHED : & str = r#"
/* FP:parse.rs-0084 */ start
/* FP:parse.rs-0085 */ ### heading
/* FP:parse.rs-0086 */ 1. ordered item
/* FP:parse.rs-0087 */ ```rust
/* FP:parse.rs-0088 */ println!("Hello, world!");
/* FP:parse.rs-0089 */ ```
/* FP:parse.rs-0090 */ `inline`
/* FP:parse.rs-0091 */ ``end``
/* FP:parse.rs-0092 */ "# ;
/* FP:parse.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0026
/* FP:parse.rs-0094 */ # [test] fn test_without_breaks () { let expected = vec ! [MdTree :: PlainText ("start") , MdTree :: ParagraphBreak , MdTree :: Heading (3 , vec ! [MdTree :: PlainText ("heading")] . into ()) , MdTree :: OrderedListItem (1 , vec ! [MdTree :: PlainText ("ordered item")] . into ()) , MdTree :: ParagraphBreak , MdTree :: CodeBlock { txt : r#"println!("Hello, world!");"# , lang : Some ("rust") } , MdTree :: ParagraphBreak , MdTree :: CodeInline ("inline") , MdTree :: PlainText (" ") , MdTree :: CodeInline ("end") ,] . into () ; let res = entrypoint (SMOOSHED) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0027
/* FP:parse.rs-0096 */ const CODE_STARTLINE : & str = r#"
/* FP:parse.rs-0097 */ start
/* FP:parse.rs-0098 */ `code`
/* FP:parse.rs-0099 */ middle
/* FP:parse.rs-0100 */ `more code`
/* FP:parse.rs-0101 */ end
/* FP:parse.rs-0102 */ "# ;
/* FP:parse.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0028
/* FP:parse.rs-0104 */ # [test] fn test_code_at_start () { let expected = vec ! [MdTree :: PlainText ("start") , MdTree :: PlainText (" ") , MdTree :: CodeInline ("code") , MdTree :: PlainText (" ") , MdTree :: PlainText ("middle") , MdTree :: PlainText (" ") , MdTree :: CodeInline ("more code") , MdTree :: PlainText (" ") , MdTree :: PlainText ("end") ,] . into () ; let res = entrypoint (CODE_STARTLINE) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0029
/* FP:parse.rs-0106 */ # [test] fn test_code_in_parens () { let expected = vec ! [MdTree :: PlainText ("(") , MdTree :: CodeInline ("Foo") , MdTree :: PlainText (")")] . into () ; let res = entrypoint ("(`Foo`)") ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0030
/* FP:parse.rs-0108 */ const LIST_WITH_SPACE : & str = "
/* FP:parse.rs-0109 */ para
/* FP:parse.rs-0110 */  * l1
/* FP:parse.rs-0111 */  * l2
/* FP:parse.rs-0112 */ " ;
/* FP:parse.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0031
/* FP:parse.rs-0114 */ # [test] fn test_list_with_space () { let expected = vec ! [MdTree :: PlainText ("para") , MdTree :: ParagraphBreak , MdTree :: UnorderedListItem (vec ! [MdTree :: PlainText ("l1")] . into ()) , MdTree :: LineBreak , MdTree :: UnorderedListItem (vec ! [MdTree :: PlainText ("l2")] . into ()) ,] . into () ; let res = entrypoint (LIST_WITH_SPACE) ; assert_eq ! (res , expected) ; }
/* FP:parse.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_CONST_0032
/* FP:parse.rs-0116 */ const SNAKE_CASE : & str = "
/* FP:parse.rs-0117 */ foo*bar*
/* FP:parse.rs-0118 */ foo**bar**
/* FP:parse.rs-0119 */ foo_bar_
/* FP:parse.rs-0120 */ foo__bar__
/* FP:parse.rs-0121 */ " ;
/* FP:parse.rs-0122 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_parse_FN_0033
/* FP:parse.rs-0123 */ # [test] fn test_snake_case () { let expected = vec ! [MdTree :: PlainText ("foo") , MdTree :: Emphasis ("bar") , MdTree :: PlainText (" ") , MdTree :: PlainText ("foo") , MdTree :: Strong ("bar") , MdTree :: PlainText (" ") , MdTree :: PlainText ("foo_bar_") , MdTree :: PlainText (" ") , MdTree :: PlainText ("foo__bar__") ,] . into () ; let res = entrypoint (SNAKE_CASE) ; assert_eq ! (res , expected) ; }