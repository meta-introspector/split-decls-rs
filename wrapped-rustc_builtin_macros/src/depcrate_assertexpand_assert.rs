// Generated macro for expand_assert (function)
macro_rules! Depcrate_assertexpand_assert {
() => {
// Module: crate::assert
// Provides: {"expand_assert"}
// Dependencies: {}
pub (crate) fn expand_assert < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , span : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let Assert { cond_expr , custom_message } = match parse_assert (cx , span , tts) { Ok (assert) => assert , Err (err) => { let guar = err . emit () ; return ExpandResult :: Ready (DummyResult :: any (span , guar)) ; } } ; let call_site_span = cx . with_call_site_ctxt (span) ; let panic_path = | | { if use_panic_2021 (span) { Path { span : call_site_span , segments : cx . std_path (& [sym :: panic , sym :: panic_2021]) . into_iter () . map (| ident | PathSegment :: from_ident (ident)) . collect () , tokens : None , } } else { Path :: from_ident (Ident :: new (sym :: panic , call_site_span)) } } ; let expr = if let Some (tokens) = custom_message { let then = cx . expr (call_site_span , ExprKind :: MacCall (Box :: new (MacCall { path : panic_path () , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (call_site_span) , delim : Delimiter :: Parenthesis , tokens , }) , })) ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } else if cx . ecfg . features . generic_assert () { context :: Context :: new (cx , call_site_span) . build (cond_expr , panic_path ()) } else { let then = cx . expr_call_global (call_site_span , cx . std_path (& [sym :: panicking , sym :: panic]) , thin_vec ! [cx . expr_str (DUMMY_SP , Symbol :: intern (& format ! ("assertion failed: {}" , pprust :: expr_to_string (& cond_expr))) ,)] ,) ; expr_if_not (cx , call_site_span , cond_expr , then , None) } ; ExpandResult :: Ready (MacEager :: expr (expr)) }
};
}
