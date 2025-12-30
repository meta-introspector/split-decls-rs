// Generated macro for UncompressedEncoding (trait)
macro_rules! DepcrateUncompressedEncoding {
() => {
// Module: crate
// Provides: {"UncompressedEncoding"}
// Dependencies: {}
# [doc = " Affine representation of a point on an elliptic curve that has a defined uncompressed"] # [doc = " encoding."] pub trait UncompressedEncoding : Sized { type Uncompressed : Default + AsRef < [u8] > + AsMut < [u8] > ; # [doc = " Attempts to deserialize an element from its uncompressed encoding."] fn from_uncompressed (bytes : & Self :: Uncompressed) -> CtOption < Self > ; # [doc = " Attempts to deserialize an uncompressed element, not checking if the element is in"] # [doc = " the correct subgroup."] # [doc = ""] # [doc = " **This is dangerous to call unless you trust the bytes you are reading; otherwise,"] # [doc = " API invariants may be broken.** Please consider using"] # [doc = " [`UncompressedEncoding::from_uncompressed`] instead."] fn from_uncompressed_unchecked (bytes : & Self :: Uncompressed) -> CtOption < Self > ; # [doc = " Converts this element into its uncompressed encoding, so long as it's not"] # [doc = " the point at infinity."] fn to_uncompressed (& self) -> Self :: Uncompressed ; }
};
}
