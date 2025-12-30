// Generated macro for expand (function)
macro_rules! Depcrate_iterexpand {
() => {
// Module: crate::iter
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let closure = match parse_closure (cx , sp , tts) { Ok (parsed) => parsed , Err (err) => { return ExpandResult :: Ready (DummyResult :: any (sp , err . emit ())) ; } } ; ExpandResult :: Ready (base :: MacEager :: expr (closure)) }
};
}
