// Generated macro for macro_2197 (macro)
macro_rules! Depcrate_io_chainmacro_2197 {
() => {
// Module: crate::io::chain
// Provides: {"macro_2197"}
// Dependencies: {}
pin_project ! { # [doc = " Reader for the [`chain`](super::AsyncReadExt::chain) method."] # [must_use = "readers do nothing unless polled"] pub struct Chain < T , U > { # [pin] first : T , # [pin] second : U , done_first : bool , } }
};
}
