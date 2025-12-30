// Generated macro for macro_420 (macro)
macro_rules! Depcrate_fut_streammacro_420 {
() => {
// Module: crate::fut::stream
// Provides: {"macro_420"}
// Dependencies: {}
pin_project ! { pub struct StreamWrap < S , A > where S : Stream , A : Actor { # [pin] stream : S , _act : PhantomData < A > } }
};
}
