// Generated macro for impl_176 (impl)
macro_rules! Depcrate_writeimpl_176 {
() => {
// Module: crate::write
// Provides: {"impl_176"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W : io :: Write > IoWrite < W > { # [doc = " Wraps an `io::Write` writer to make it compatible with [`Write`](trait.Write.html)"] pub fn new (w : W) -> IoWrite < W > { IoWrite (w) } }
};
}
