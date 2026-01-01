/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: path :: PathBuf ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_complete :: source_map :: { FilePathMapping , SourceMap } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: rustc_complete :: symbol :: sym ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: rustc_complete :: { BytePos , Span } ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_USE_0005
/* FP:tests.rs-0010 */ use super :: { DocFragment , DocFragmentKind , source_span_for_markdown_range_inner } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn single_backtick () { let sm = SourceMap :: new (FilePathMapping :: empty ()) ; sm . new_source_file (PathBuf :: from ("foo.rs") . into () , r#"#[doc = "`"] fn foo() {}"# . to_string ()) ; let (span , _) = source_span_for_markdown_range_inner (& sm , "`" , & (0 .. 1) , & [DocFragment { span : Span :: with_root_ctxt (BytePos (8) , BytePos (11)) , item_id : None , kind : DocFragmentKind :: RawDoc , doc : sym :: empty , indent : 0 , from_expansion : false , }] ,) . unwrap () ; assert_eq ! (span . lo () , BytePos (9)) ; assert_eq ! (span . hi () , BytePos (10)) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_rustdoc_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn utf8 () { let sm = SourceMap :: new (FilePathMapping :: empty ()) ; sm . new_source_file (PathBuf :: from ("foo.rs") . into () , r#"#[doc = "⚠"] fn foo() {}"# . to_string ()) ; let (span , _) = source_span_for_markdown_range_inner (& sm , "⚠" , & (0 .. 3) , & [DocFragment { span : Span :: with_root_ctxt (BytePos (8) , BytePos (14)) , item_id : None , kind : DocFragmentKind :: RawDoc , doc : sym :: empty , indent : 0 , from_expansion : false , }] ,) . unwrap () ; assert_eq ! (span . lo () , BytePos (9)) ; assert_eq ! (span . hi () , BytePos (12)) ; }