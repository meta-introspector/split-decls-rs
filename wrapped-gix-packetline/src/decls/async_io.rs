macro_rules! async_io {
    () => {
        # [doc = ""] # [cfg (feature = "async-io")] pub mod async_io { # [doc = ""] pub mod encode ; mod read ; pub use read :: StreamingPeekableIter ; mod sidebands ; pub use sidebands :: WithSidebands ; mod write ; pub use write :: Writer ; }
    };
}

async_io!();