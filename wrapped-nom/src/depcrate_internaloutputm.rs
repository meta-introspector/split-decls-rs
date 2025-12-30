// Generated macro for OutputM (struct)
macro_rules! Depcrate_internalOutputM {
() => {
// Module: crate::internal
// Provides: {"OutputM"}
// Dependencies: {}
# [doc = " Holds the parser execution modifiers: output [Mode], error [Mode] and"] # [doc = " streaming behaviour for input data"] pub struct OutputM < M : Mode , EM : Mode , S : IsStreaming > { m : PhantomData < M > , em : PhantomData < EM > , s : PhantomData < S > , }
};
}
