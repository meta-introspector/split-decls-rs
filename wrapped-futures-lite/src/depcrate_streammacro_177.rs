// Generated macro for macro_177 (macro)
macro_rules! Depcrate_streammacro_177 {
() => {
// Module: crate::stream
// Provides: {"macro_177"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::enumerate()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Enumerate < S > { # [pin] stream : S , i : usize , } }
};
}
