/* FP:trace_macros.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_trace_macros_USE_0001
/* FP:trace_macros.rs-0002 */ use crate :: rustc_complete :: tokenstream :: { TokenStream , TokenTree } ;
/* FP:trace_macros.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_trace_macros_USE_0002
/* FP:trace_macros.rs-0004 */ use crate :: rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;
/* FP:trace_macros.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_trace_macros_USE_0003
/* FP:trace_macros.rs-0006 */ use crate :: rustc_complete :: { Span , kw } ;
/* FP:trace_macros.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_trace_macros_USE_0004
/* FP:trace_macros.rs-0008 */ use crate :: errors ;
/* FP:trace_macros.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_trace_macros_FN_0005
/* FP:trace_macros.rs-0010 */ pub (crate) fn expand_trace_macros (cx : & mut ExtCtxt < '_ > , sp : Span , tt : TokenStream ,) -> MacroExpanderResult < 'static > { let mut iter = tt . iter () ; let mut err = false ; let value = match iter . next () { Some (TokenTree :: Token (token , _)) if token . is_keyword (kw :: True) => true , Some (TokenTree :: Token (token , _)) if token . is_keyword (kw :: False) => false , _ => { err = true ; false } } ; err |= iter . next () . is_some () ; if err { cx . dcx () . emit_err (errors :: TraceMacros { span : sp }) ; } else { cx . set_trace_macros (value) ; } ExpandResult :: Ready (DummyResult :: any_valid (sp)) }