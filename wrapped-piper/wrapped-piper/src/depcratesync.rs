// Generated macro for sync (module)
macro_rules! Depcratesync {
() => {
// Module: crate
// Provides: {"sync"}
// Dependencies: {}
mod sync { # [cfg (not (feature = "portable-atomic"))] pub use core :: sync :: atomic ; # [cfg (not (feature = "portable-atomic"))] pub use alloc :: sync :: Arc ; # [cfg (feature = "portable-atomic")] pub use portable_atomic_crate as atomic ; # [cfg (feature = "portable-atomic")] pub use portable_atomic_util :: Arc ; }
};
}
