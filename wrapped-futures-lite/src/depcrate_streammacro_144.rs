// Generated macro for macro_144 (macro)
macro_rules! Depcrate_streammacro_144 {
() => {
// Module: crate::stream
// Provides: {"macro_144"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::then()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Then < S , F , Fut > { # [pin] stream : S , # [pin] future : Option < Fut >, f : F , } }
};
}
