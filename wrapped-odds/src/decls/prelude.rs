macro_rules! prelude {
    () => {
        # [doc = " prelude of often used traits and functions"] pub mod prelude { # [doc (no_inline)] pub use crate :: fix ; pub use crate :: slice :: SliceFind ; pub use crate :: string :: StrChunksWindows ; pub use crate :: string :: StrExt ; # [doc (no_inline)] pub use crate :: IndexRange ; # [cfg (feature = "std-string")] pub use string :: StringExt ; # [cfg (feature = "std-vec")] pub use vec :: { vec , VecExt } ; }
    };
}

prelude!()