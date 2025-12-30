// Generated macro for macro_136 (macro)
macro_rules! Depcrate_streammacro_136 {
() => {
// Module: crate::stream
// Provides: {"macro_136"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::fuse()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Fuse < S > { # [pin] stream : S , done : bool , } }
};
}
