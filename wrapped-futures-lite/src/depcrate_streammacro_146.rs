// Generated macro for macro_146 (macro)
macro_rules! Depcrate_streammacro_146 {
() => {
// Module: crate::stream
// Provides: {"macro_146"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::filter()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Filter < S , P > { # [pin] stream : S , predicate : P , } }
};
}
