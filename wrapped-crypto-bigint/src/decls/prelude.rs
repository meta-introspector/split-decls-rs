macro_rules! deps {
    () => {
        ArrayEncoding!();
        ArrayDecoding!();
    };
}

macro_rules! prelude {
    () => {
        deps!();
        # [doc = " Import prelude for this crate: includes important traits."] pub mod prelude { # [cfg (feature = "hybrid-array")] pub use crate :: array :: { ArrayDecoding , ArrayEncoding } ; pub use crate :: traits :: * ; }
    };
}

prelude!();