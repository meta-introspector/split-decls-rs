// Generated macro for macro_159 (macro)
macro_rules! Depcrate_streammacro_159 {
() => {
// Module: crate::stream
// Provides: {"macro_159"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::take_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct TakeWhile < S , P > { # [pin] stream : S , predicate : P , } }
};
}
