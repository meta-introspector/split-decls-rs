// Generated macro for impl_182 (impl)
macro_rules! Depcrate_writeimpl_182 {
() => {
// Module: crate::write
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < 'a , W : Write > FmtWrite < 'a , W > { # [doc = " Wraps an `fmt::Write` writer to make it compatible with [`Write`](trait.Write.html)"] pub fn new (w : & 'a mut W) -> FmtWrite < 'a , W > { FmtWrite (w) } }
};
}
