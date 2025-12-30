// Generated macro for snippet_with_applicability_sess (function)
macro_rules! Depcrate_sourcesnippet_with_applicability_sess {
() => {
// Module: crate::source
// Provides: {"snippet_with_applicability_sess"}
// Dependencies: {}
fn snippet_with_applicability_sess < 'a > (sess : & Session , span : Span , default : & 'a str , applicability : & mut Applicability ,) -> Cow < 'a , str > { if * applicability != Applicability :: Unspecified && span . from_expansion () { * applicability = Applicability :: MaybeIncorrect ; } snippet_opt (sess , span) . map_or_else (| | { if * applicability == Applicability :: MachineApplicable { * applicability = Applicability :: HasPlaceholders ; } Cow :: Borrowed (default) } , From :: from ,) }
};
}
