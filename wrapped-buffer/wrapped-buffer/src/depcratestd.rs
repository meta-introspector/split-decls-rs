// Generated macro for std (module)
macro_rules! Depcratestd {
() => {
// Module: crate
// Provides: {"std"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod std { # [allow (unused_imports)] pub use crate :: { alloc :: { borrow , boxed , collections , string , vec } , core :: { convert , fmt , hash , marker , mem , ops , result , str } , } ; # [cfg (feature = "std")] pub use libstd :: error ; }
};
}
