macro_rules! prelude {
    () => {
        # [doc = " Import prelude for this crate: includes important traits."] pub mod prelude { # [cfg (feature = "hybrid-array")] pub use crate :: array :: { ArrayDecoding , ArrayEncoding } ; pub use crate :: traits :: * ; }
    };
}

prelude!()