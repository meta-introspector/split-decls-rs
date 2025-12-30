// Generated macro for snippet_with_applicability (function)
macro_rules! Depcrate_sourcesnippet_with_applicability {
() => {
// Module: crate::source
// Provides: {"snippet_with_applicability"}
// Dependencies: {}
# [doc = " Same as [`snippet`], but it adapts the applicability level by following rules:"] # [doc = ""] # [doc = " - Applicability level `Unspecified` will never be changed."] # [doc = " - If the span is inside a macro, change the applicability level to `MaybeIncorrect`."] # [doc = " - If the default value is used and the applicability level is `MachineApplicable`, change it to"] # [doc = "   `HasPlaceholders`"] pub fn snippet_with_applicability < 'a > (sess : & impl HasSession , span : Span , default : & 'a str , applicability : & mut Applicability ,) -> Cow < 'a , str > { snippet_with_applicability_sess (sess . sess () , span , default , applicability) }
};
}
