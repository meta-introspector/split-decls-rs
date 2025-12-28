macro_rules! blocking_io {
    () => {
        # [doc = ""] # [cfg (feature = "blocking-io")] pub mod blocking_io { # [doc = ""] pub mod encode ; mod read ; pub use read :: StreamingPeekableIter ; mod sidebands ; pub use sidebands :: WithSidebands ; mod write ; pub use write :: Writer ; }
    };
}

blocking_io!()