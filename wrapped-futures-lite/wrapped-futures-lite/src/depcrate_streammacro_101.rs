// Generated macro for macro_101 (macro)
macro_rules! Depcrate_streammacro_101 {
() => {
// Module: crate::stream
// Provides: {"macro_101"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_unfold()`] function."] # [derive (Clone)] # [must_use = "streams do nothing unless polled"] pub struct TryUnfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
};
}
