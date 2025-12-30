// Generated macro for macro_157 (macro)
macro_rules! Depcrate_streammacro_157 {
() => {
// Module: crate::stream
// Provides: {"macro_157"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::take()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Take < S > { # [pin] stream : S , n : usize , } }
};
}
