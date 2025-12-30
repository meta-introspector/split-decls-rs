// Generated macro for async_io (module)
macro_rules! Depcrateasync_io {
() => {
// Module: crate
// Provides: {"async_io"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "async-io")] pub mod async_io { # [doc = ""] pub mod encode ; mod read ; pub use read :: StreamingPeekableIter ; mod sidebands ; pub use sidebands :: WithSidebands ; mod write ; pub use write :: Writer ; }
};
}
