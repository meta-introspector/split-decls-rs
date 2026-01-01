/* FP:iter.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_USE_0001
/* FP:iter.rs-0002 */ use crate :: rustc_complete :: tokenstream :: TokenStream ;
/* FP:iter.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_USE_0002
/* FP:iter.rs-0004 */ use crate :: rustc_complete :: { CoroutineKind , DUMMY_NODE_ID , Expr , ast , token } ;
/* FP:iter.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_USE_0003
/* FP:iter.rs-0006 */ use crate :: rustc_complete :: PResult ;
/* FP:iter.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_USE_0004
/* FP:iter.rs-0008 */ use crate :: rustc_expand :: base :: { self , DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;
/* FP:iter.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_USE_0005
/* FP:iter.rs-0010 */ use crate :: rustc_complete :: Span ;
/* FP:iter.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_FN_0006
/* FP:iter.rs-0012 */ pub (crate) fn expand < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let closure = match parse_closure (cx , sp , tts) { Ok (parsed) => parsed , Err (err) => { return ExpandResult :: Ready (DummyResult :: any (sp , err . emit ())) ; } } ; ExpandResult :: Ready (base :: MacEager :: expr (closure)) }
/* FP:iter.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_iter_FN_0007
/* FP:iter.rs-0014 */ fn parse_closure < 'a > (cx : & mut ExtCtxt < 'a > , span : Span , stream : TokenStream ,) -> PResult < 'a , Box < Expr > > { let mut closure_parser = cx . new_parser_from_tts (stream) ; let coroutine_kind = Some (CoroutineKind :: Gen { span , closure_id : DUMMY_NODE_ID , return_impl_trait_id : DUMMY_NODE_ID , }) ; let mut closure = closure_parser . parse_expr () ? ; match & mut closure . kind { ast :: ExprKind :: Closure (c) => { if let Some (kind) = c . coroutine_kind { cx . dcx () . span_err (kind . span () , "only plain closures allowed in `iter!`") ; } c . coroutine_kind = coroutine_kind ; if closure_parser . token != token :: Eof { closure_parser . unexpected () ? ; } Ok (closure) } _ => { cx . dcx () . span_err (closure . span , "`iter!` body must be a closure") ; Err (closure_parser . unexpected () . unwrap_err ()) } } }