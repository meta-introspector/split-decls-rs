// Generated macro for macro_175 (macro)
macro_rules! Depcrate_streammacro_175 {
() => {
// Module: crate::stream
// Provides: {"macro_175"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::cycle()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cycle < S > { orig : S , # [pin] stream : S , } }
};
}
