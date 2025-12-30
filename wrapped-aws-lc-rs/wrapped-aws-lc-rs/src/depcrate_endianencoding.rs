// Generated macro for Encoding (trait)
macro_rules! Depcrate_endianEncoding {
() => {
// Module: crate::endian
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " An `Encoding` of a type `T` can be converted to/from its byte"] # [doc = " representation without any byte swapping or other computation."] # [doc = ""] # [doc = " The `Self: Copy` constraint addresses `clippy::declare_interior_mutable_const`."] pub trait Encoding < T > : From < T > + Into < T > where Self : Copy , { const ZERO : Self ; }
};
}
