// Generated macro for macro_97 (macro)
macro_rules! Depcrate_streammacro_97 {
() => {
// Module: crate::stream
// Provides: {"macro_97"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`unfold()`] function."] # [derive (Clone)] # [must_use = "streams do nothing unless polled"] pub struct Unfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
};
}
