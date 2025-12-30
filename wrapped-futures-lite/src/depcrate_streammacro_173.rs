// Generated macro for macro_173 (macro)
macro_rules! Depcrate_streammacro_173 {
() => {
// Module: crate::stream
// Provides: {"macro_173"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::copied()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Copied < S > { # [pin] stream : S , } }
};
}
