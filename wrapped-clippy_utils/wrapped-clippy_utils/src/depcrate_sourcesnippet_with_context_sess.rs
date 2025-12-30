// Generated macro for snippet_with_context_sess (function)
macro_rules! Depcrate_sourcesnippet_with_context_sess {
() => {
// Module: crate::source
// Provides: {"snippet_with_context_sess"}
// Dependencies: {}
fn snippet_with_context_sess < 'a > (sess : & Session , span : Span , outer : SyntaxContext , default : & 'a str , applicability : & mut Applicability ,) -> (Cow < 'a , str > , bool) { if span . desugaring_kind () == Some (DesugaringKind :: RangeExpr) && span . parent_callsite () . unwrap () . ctxt () == outer { return (snippet_with_applicability_sess (sess , span , default , applicability) , false ,) ; } let (span , is_macro_call) = walk_span_to_context (span , outer) . map_or_else (| | { if * applicability != Applicability :: Unspecified { * applicability = Applicability :: MaybeIncorrect ; } (span , false) } , | outer_span | (outer_span , span . ctxt () != outer) ,) ; (snippet_with_applicability_sess (sess , span , default , applicability) , is_macro_call ,) }
};
}
