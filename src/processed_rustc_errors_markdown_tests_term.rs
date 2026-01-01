/* FP:term.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_USE_0001
/* FP:term.rs-0002 */ use std :: io :: BufWriter ;
/* FP:term.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_USE_0002
/* FP:term.rs-0004 */ use std :: path :: PathBuf ;
/* FP:term.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_USE_0003
/* FP:term.rs-0006 */ use termcolor :: { BufferWriter , ColorChoice } ;
/* FP:term.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_USE_0004
/* FP:term.rs-0008 */ use super :: * ;
/* FP:term.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_CONST_0005
/* FP:term.rs-0010 */ const INPUT : & str = include_str ! ("input.md") ;
/* FP:term.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_CONST_0006
/* FP:term.rs-0012 */ const OUTPUT_PATH : & [& str] = & [env ! ("CARGO_MANIFEST_DIR") , "src" , "markdown" , "tests" , "output.stdout"] ;
/* FP:term.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_CONST_0007
/* FP:term.rs-0014 */ const TEST_WIDTH : usize = 80 ;
/* FP:term.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_CONST_0008
/* FP:term.rs-0016 */ const TXT : & str = r"Lorem ipsum dolor sit amet, consecteturadipiscingelit.
/* FP:term.rs-0017 */ Fusce-id-urna-sollicitudin, pharetra nisl nec, lobortis tellus. In at
/* FP:term.rs-0018 */ metus hendrerit, tincidunteratvel, ultrices turpis. Curabitur_risus_sapien,
/* FP:term.rs-0019 */ porta-sed-nunc-sed, ultricesposuerelacus. Sed porttitor quis
/* FP:term.rs-0020 */ dolor non venenatis. Aliquam ut. " ;
/* FP:term.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_CONST_0009
/* FP:term.rs-0022 */ const WRAPPED : & str = r"Lorem ipsum dolor sit amet, consecteturadipiscingelit. Fusce-id-urna-
/* FP:term.rs-0023 */ sollicitudin, pharetra nisl nec, lobortis tellus. In at metus hendrerit,
/* FP:term.rs-0024 */ tincidunteratvel, ultrices turpis. Curabitur_risus_sapien, porta-sed-nunc-sed,
/* FP:term.rs-0025 */ ultricesposuerelacus. Sed porttitor quis dolor non venenatis. Aliquam ut. Lorem
/* FP:term.rs-0026 */     ipsum dolor sit amet, consecteturadipiscingelit. Fusce-id-urna-
/* FP:term.rs-0027 */     sollicitudin, pharetra nisl nec, lobortis tellus. In at metus hendrerit,
/* FP:term.rs-0028 */     tincidunteratvel, ultrices turpis. Curabitur_risus_sapien, porta-sed-nunc-
/* FP:term.rs-0029 */     sed, ultricesposuerelacus. Sed porttitor quis dolor non venenatis. Aliquam
/* FP:term.rs-0030 */     ut. Sample link lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet,
/* FP:term.rs-0031 */ consecteturadipiscingelit. Fusce-id-urna-sollicitudin, pharetra nisl nec,
/* FP:term.rs-0032 */ lobortis tellus. In at metus hendrerit, tincidunteratvel, ultrices turpis.
/* FP:term.rs-0033 */ Curabitur_risus_sapien, porta-sed-nunc-sed, ultricesposuerelacus. Sed porttitor
/* FP:term.rs-0034 */ quis dolor non venenatis. Aliquam ut. " ;
/* FP:term.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_FN_0010
/* FP:term.rs-0036 */ # [test] fn test_wrapping_write () { WIDTH . with (| w | w . set (TEST_WIDTH)) ; let mut buf = BufWriter :: new (Vec :: new ()) ; let txt = TXT . replace ("-\n" , "-") . replace ("_\n" , "_") . replace ('\n' , " ") . replace ("    " , "") ; write_wrapping (& mut buf , & txt , 0 , None) . unwrap () ; write_wrapping (& mut buf , & txt , 4 , None) . unwrap () ; write_wrapping (& mut buf , "Sample link lorem ipsum dolor sit amet. " , 4 , Some ("link-address-placeholder") ,) . unwrap () ; write_wrapping (& mut buf , & txt , 0 , None) . unwrap () ; let out = String :: from_utf8 (buf . into_inner () . unwrap ()) . unwrap () ; let out = out . replace ("\x1b\\" , "") . replace ('\x1b' , "") . replace ("]8;;" , "") . replace ("link-address-placeholder" , "") ; for line in out . lines () { assert ! (line . len () <= TEST_WIDTH , "line length\n'{line}'") } assert_eq ! (out , WRAPPED) ; }
/* FP:term.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_errors_src_markdown_tests_term_FN_0011
/* FP:term.rs-0038 */ # [test] fn test_output () { let bless = std :: env :: var_os ("RUSTC_BLESS") . is_some_and (| v | v != "0") ; let ast = MdStream :: parse_str (INPUT) ; let bufwtr = BufferWriter :: stderr (ColorChoice :: Always) ; let mut buffer = bufwtr . buffer () ; ast . write_termcolor_buf (& mut buffer) . unwrap () ; let mut blessed = PathBuf :: new () ; blessed . extend (OUTPUT_PATH) ; if bless { std :: fs :: write (& blessed , buffer . into_inner ()) . unwrap () ; eprintln ! ("blessed output at {}" , blessed . display ()) ; } else { let output = buffer . into_inner () ; if std :: fs :: read (blessed) . unwrap () != output { let mut out = std :: io :: stdout () ; out . write_all (b"\n\nMarkdown output did not match. Expected:\n") . unwrap () ; out . write_all (& output) . unwrap () ; out . write_all (b"\n\n") . unwrap () ; panic ! ("markdown output mismatch") ; } } }