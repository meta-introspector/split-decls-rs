/* FP:assert.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_MOD_0001
/* FP:assert.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0002
/* FP:assert.rs-0004 */ use crate :: rustc_complete :: token :: Delimiter ;
/* FP:assert.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0003
/* FP:assert.rs-0006 */ use crate :: rustc_complete :: tokenstream :: { DelimSpan , TokenStream } ;
/* FP:assert.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0004
/* FP:assert.rs-0008 */ use crate :: rustc_complete :: { DelimArgs , Expr , ExprKind , MacCall , Path , PathSegment , UnOp , token } ;
/* FP:assert.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0005
/* FP:assert.rs-0010 */ use rustc_ast_pretty :: pprust ;
/* FP:assert.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0006
/* FP:assert.rs-0012 */ use crate :: rustc_complete :: PResult ;
/* FP:assert.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0007
/* FP:assert.rs-0014 */ use crate :: rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacEager , MacroExpanderResult } ;
/* FP:assert.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0008
/* FP:assert.rs-0016 */ use crate :: rustc_parse :: exp ;
/* FP:assert.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0009
/* FP:assert.rs-0018 */ use crate :: rustc_parse :: parser :: Parser ;
/* FP:assert.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0010
/* FP:assert.rs-0020 */ use crate :: rustc_complete :: { DUMMY_SP , Ident , Span , Symbol , sym } ;
/* FP:assert.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0011
/* FP:assert.rs-0022 */ use thin_vec :: thin_vec ;
/* FP:assert.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0012
/* FP:assert.rs-0024 */ use crate :: edition_panic :: use_panic_2021 ;
/* FP:assert.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_USE_0013
/* FP:assert.rs-0026 */ use crate :: errors ;
/* FP:assert.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_FN_0014
/* FP:assert.rs-0028 */ pub (crate) fn expand_assert < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , span : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let Assert { cond_expr , custom_message } = match parse_assert (cx , span , tts) { Ok (assert) => assert , Err (err) => { let guar = err . emit () ; return ExpandResult :: Ready (DummyResult :: any (span , guar)) ; } } ; let call_site_span = cx . with_call_site_ctxt (span) ; let panic_path = | | { if use_panic_2021 (span) { Path { span : call_site_span , segments : cx . std_path (& [sym :: panic , sym :: panic_2021]) . into_iter () . map (| ident | PathSegment :: from_ident (ident)) . collect () , tokens : None , } } else { Path :: from_ident (Ident :: new (sym :: panic , call_site_span)) } } ; let expr = if let Some (tokens) = custom_message { let then = cx . expr (call_site_span , ExprKind :: MacCall (Box :: new (MacCall { path : panic_path () , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (call_site_span) , delim : Delimiter :: Parenthesis , tokens , }) , })) ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } else if cx . ecfg . features . generic_assert () { context :: Context :: new (cx , call_site_span) . build (cond_expr , panic_path ()) } else { let then = cx . expr_call_global (call_site_span , cx . std_path (& [sym :: panicking , sym :: panic]) , thin_vec ! [cx . expr_str (DUMMY_SP , Symbol :: intern (& format ! ("assertion failed: {}" , pprust :: expr_to_string (& cond_expr))) ,)] ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } ; ExpandResult :: Ready (MacEager :: expr (expr)) }
/* FP:assert.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_STRUCT_0015
/* FP:assert.rs-0030 */ struct Assert { cond_expr : Box < Expr > , custom_message : Option < TokenStream > , }
/* FP:assert.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_FN_0016
/* FP:assert.rs-0032 */ fn expr_if_not (cx : & ExtCtxt < '_ > , span : Span , cond : Box < Expr > , then : Box < Expr > , els : Option < Box < Expr > > ,) -> Box < Expr > { cx . expr_if (span , cx . expr (span , ExprKind :: Unary (UnOp :: Not , cond)) , then , els) }
/* FP:assert.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_FN_0017
/* FP:assert.rs-0034 */ fn parse_assert < 'a > (cx : & ExtCtxt < 'a > , sp : Span , stream : TokenStream) -> PResult < 'a , Assert > { let mut parser = cx . new_parser_from_tts (stream) ; if parser . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: AssertRequiresBoolean { span : sp })) ; } let cond_expr = parser . parse_expr () ? ; if parser . token == token :: Semi { cx . dcx () . emit_err (errors :: AssertRequiresExpression { span : sp , token : parser . token . span }) ; parser . bump () ; } let custom_message = if let token :: Literal (token :: Lit { kind : token :: Str , .. }) = parser . token . kind { let comma = parser . prev_token . span . shrink_to_hi () ; cx . dcx () . emit_err (errors :: AssertMissingComma { span : parser . token . span , comma }) ; parse_custom_message (& mut parser) } else if parser . eat (exp ! (Comma)) { parse_custom_message (& mut parser) } else { None } ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (Assert { cond_expr , custom_message }) }
/* FP:assert.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_assert_FN_0018
/* FP:assert.rs-0036 */ fn parse_custom_message (parser : & mut Parser < '_ >) -> Option < TokenStream > { let ts = parser . parse_tokens () ; if ! ts . is_empty () { Some (ts) } else { None } }