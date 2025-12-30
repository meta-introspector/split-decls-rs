// Generated macro for current (module)
macro_rules! Depcratecurrent {
() => {
// Module: crate
// Provides: {"current"}
// Dependencies: {}
# [doc = " A short-cut to the architecture (bits32 or bits64) this crate was compiled for."] pub mod current { # [cfg (target_arch = "x86")] pub use crate :: bits32 :: * ; # [cfg (target_arch = "x86_64")] pub use crate :: bits64 :: * ; }
};
}
