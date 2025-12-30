// Generated macro for LenType (trait)
macro_rules! Depcrate_len_typeLenType {
() => {
// Module: crate::len_type
// Provides: {"LenType"}
// Dependencies: {}
# [doc = " A sealed trait representing a valid type to use as a length for a container."] # [doc = ""] # [doc = " This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`."] # [cfg (not (feature = "zeroize"))] pub trait LenType : Sealed { }
};
}
