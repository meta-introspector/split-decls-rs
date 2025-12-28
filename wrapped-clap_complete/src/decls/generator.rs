macro_rules! generator {
    () => {
        # [doc = " Deprecated, see [`aot`]"] pub mod generator { pub use crate :: aot :: generate ; pub use crate :: aot :: generate_to ; pub use crate :: aot :: utils ; pub use crate :: aot :: Generator ; }
    };
}

generator!()