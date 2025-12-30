// Generated macro for BitStringLike (trait)
macro_rules! Depcrate_private_key_infoBitStringLike {
() => {
// Module: crate::private_key_info
// Provides: {"BitStringLike"}
// Dependencies: {}
# [doc = " [`BitStringLike`] marks object that will act like a BitString."] # [doc = ""] # [doc = " It will allow to get a [`BitStringRef`] that points back to the underlying bytes."] pub trait BitStringLike { fn as_bit_string (& self) -> BitStringRef < '_ > ; }
};
}
