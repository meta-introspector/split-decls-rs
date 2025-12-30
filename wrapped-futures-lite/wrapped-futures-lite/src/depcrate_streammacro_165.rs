// Generated macro for macro_165 (macro)
macro_rules! Depcrate_streammacro_165 {
() => {
// Module: crate::stream
// Provides: {"macro_165"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::skip_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct SkipWhile < S , P > { # [pin] stream : S , predicate : Option < P >, } }
};
}
