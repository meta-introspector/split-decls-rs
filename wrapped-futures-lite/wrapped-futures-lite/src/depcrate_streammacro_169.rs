// Generated macro for macro_169 (macro)
macro_rules! Depcrate_streammacro_169 {
() => {
// Module: crate::stream
// Provides: {"macro_169"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::chain()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chain < S , U > { # [pin] first : Fuse < S >, # [pin] second : Fuse < U >, } }
};
}
