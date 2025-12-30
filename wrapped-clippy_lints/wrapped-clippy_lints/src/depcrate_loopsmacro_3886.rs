// Generated macro for macro_3886 (macro)
macro_rules! Depcrate_loopsmacro_3886 {
() => {
// Module: crate::loops
// Provides: {"macro_3886"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for for-loops that manually copy items between"] # [doc = " slices that could be optimized by having a memcpy."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is not as fast as a memcpy."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let src = vec![1];"] # [doc = " # let mut dst = vec![0; 65];"] # [doc = " for i in 0..src.len() {"] # [doc = "     dst[i + 64] = src[i];"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let src = vec![1];"] # [doc = " # let mut dst = vec![0; 65];"] # [doc = " dst[64..(src.len() + 64)].clone_from_slice(&src[..]);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MANUAL_MEMCPY , perf , "manually copying items between slices" }
};
}
