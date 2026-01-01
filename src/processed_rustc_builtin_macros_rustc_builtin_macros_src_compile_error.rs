/* FP:compile_error.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_compile_error_USE_0001
/* FP:compile_error.rs-0002 */ use crate :: rustc_complete :: tokenstream :: TokenStream ;
/* FP:compile_error.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_compile_error_USE_0002
/* FP:compile_error.rs-0004 */ use crate :: rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;
/* FP:compile_error.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_compile_error_USE_0003
/* FP:compile_error.rs-0006 */ use crate :: rustc_complete :: Span ;
/* FP:compile_error.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_compile_error_USE_0004
/* FP:compile_error.rs-0008 */ use crate :: util :: get_single_str_from_tts ;
/* FP:compile_error.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_compile_error_FN_0005
/* FP:compile_error.rs-0010 */ pub (crate) fn expand_compile_error < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let ExpandResult :: Ready (mac) = get_single_str_from_tts (cx , sp , tts , "compile_error!") else { return ExpandResult :: Retry (()) ; } ; let var = match mac { Ok (var) => var , Err (guar) => return ExpandResult :: Ready (DummyResult :: any (sp , guar)) , } ; # [expect (rustc :: diagnostic_outside_of_impl , reason = "diagnostic message is specified by user")] # [expect (rustc :: untranslatable_diagnostic , reason = "diagnostic message is specified by user")] let guar = cx . dcx () . span_err (sp , var . to_string ()) ; ExpandResult :: Ready (DummyResult :: any (sp , guar)) }