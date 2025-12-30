// Generated macro for expand (function)
macro_rules! Depcrate_edition_panicexpand {
() => {
// Module: crate::edition_panic
// Provides: {"expand"}
// Dependencies: {}
fn expand < 'cx > (mac : rustc_span :: Symbol , cx : & 'cx ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let sp = cx . with_call_site_ctxt (sp) ; ExpandResult :: Ready (MacEager :: expr (cx . expr (sp , ExprKind :: MacCall (Box :: new (MacCall { path : Path { span : sp , segments : cx . std_path (& [sym :: panic , mac]) . into_iter () . map (| ident | PathSegment :: from_ident (ident)) . collect () , tokens : None , } , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (sp) , delim : Delimiter :: Parenthesis , tokens : tts , }) , })) ,) ,)) }
};
}
