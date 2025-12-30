// Generated macro for ArrayEncoding (trait)
macro_rules! Depcrate_endianArrayEncoding {
() => {
// Module: crate::endian
// Provides: {"ArrayEncoding"}
// Dependencies: {}
# [doc = " Work around the inability to implement `AsRef` for arrays of `Encoding`s"] # [doc = " due to the coherence rules."] pub trait ArrayEncoding < T > { fn as_byte_array (& self) -> & T ; }
};
}
