// Generated macro for FromArray (trait)
macro_rules! Depcrate_endianFromArray {
() => {
// Module: crate::endian
// Provides: {"FromArray"}
// Dependencies: {}
# [doc = " Work around the inability to implement `from` for arrays of `Encoding`s"] # [doc = " due to the coherence rules."] pub trait FromArray < const N : usize , T > where Self : Sized , { fn from_array (a : & [T ; N]) -> [Self ; N] ; }
};
}
