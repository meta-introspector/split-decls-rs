// Generated macro for macro_138 (macro)
macro_rules! Depcrate_streammacro_138 {
() => {
// Module: crate::stream
// Provides: {"macro_138"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Map < S , F > { # [pin] stream : S , f : F , } }
};
}
