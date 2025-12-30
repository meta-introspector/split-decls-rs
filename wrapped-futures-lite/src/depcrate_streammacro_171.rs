// Generated macro for macro_171 (macro)
macro_rules! Depcrate_streammacro_171 {
() => {
// Module: crate::stream
// Provides: {"macro_171"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::cloned()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cloned < S > { # [pin] stream : S , } }
};
}
