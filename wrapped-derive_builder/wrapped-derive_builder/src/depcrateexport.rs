// Generated macro for export (module)
macro_rules! Depcrateexport {
() => {
// Module: crate
// Provides: {"export"}
// Dependencies: {}
# [doc (hidden)] pub mod export { pub mod core { # [cfg (feature = "alloc")] pub use :: alloc :: string ; # [cfg (not (feature = "std"))] pub use core :: * ; # [cfg (feature = "std")] pub use std :: * ; } }
};
}
