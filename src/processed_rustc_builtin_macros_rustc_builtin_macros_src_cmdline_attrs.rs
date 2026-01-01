/* FP:cmdline_attrs.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0001
/* FP:cmdline_attrs.rs-0002 */ use crate :: rustc_complete :: { self as ast } ;
/* FP:cmdline_attrs.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0002
/* FP:cmdline_attrs.rs-0004 */ use crate :: rustc_complete :: Diag ;
/* FP:cmdline_attrs.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0003
/* FP:cmdline_attrs.rs-0006 */ use crate :: rustc_parse :: parser :: attr :: InnerAttrPolicy ;
/* FP:cmdline_attrs.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0004
/* FP:cmdline_attrs.rs-0008 */ use crate :: rustc_parse :: { parse_in , source_str_to_stream } ;
/* FP:cmdline_attrs.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0005
/* FP:cmdline_attrs.rs-0010 */ use crate :: rustc_complete :: parse :: ParseSess ;
/* FP:cmdline_attrs.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_USE_0006
/* FP:cmdline_attrs.rs-0012 */ use crate :: rustc_complete :: FileName ;
/* FP:cmdline_attrs.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cmdline_attrs_FN_0007
/* FP:cmdline_attrs.rs-0014 */ pub fn inject (krate : & mut ast :: Crate , psess : & ParseSess , attrs : & [String]) { for raw_attr in attrs { let source = format ! ("#[{raw_attr}]") ; let parse = | | -> Result < ast :: Attribute , Vec < Diag < '_ > > > { let tokens = source_str_to_stream (psess , FileName :: cli_crate_attr_source_code (raw_attr) , source , None ,) ? ; parse_in (psess , tokens , "<crate attribute>" , | p | { p . parse_attribute (InnerAttrPolicy :: Permitted) }) . map_err (| e | vec ! [e]) } ; let meta = match parse () { Ok (meta) => meta , Err (errs) => { for err in errs { err . emit () ; } continue ; } } ; krate . attrs . push (meta) ; } }