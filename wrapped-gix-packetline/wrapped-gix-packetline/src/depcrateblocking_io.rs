// Generated macro for blocking_io (module)
macro_rules! Depcrateblocking_io {
() => {
// Module: crate
// Provides: {"blocking_io"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "blocking-io")] pub mod blocking_io { # [doc = ""] pub mod encode ; mod read ; pub use read :: StreamingPeekableIter ; mod sidebands ; pub use sidebands :: WithSidebands ; mod write ; pub use write :: Writer ; }
};
}
