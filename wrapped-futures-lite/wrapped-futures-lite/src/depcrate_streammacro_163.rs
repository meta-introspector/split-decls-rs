// Generated macro for macro_163 (macro)
macro_rules! Depcrate_streammacro_163 {
() => {
// Module: crate::stream
// Provides: {"macro_163"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::skip()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Skip < S > { # [pin] stream : S , n : usize , } }
};
}
