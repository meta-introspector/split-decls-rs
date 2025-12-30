// Generated macro for impl_10 (impl)
macro_rules! Depcrate_attrimpl_10 {
() => {
// Module: crate::attr
// Provides: {"impl_10"}
// Dependencies: {}
impl MessageDigest { # [doc = " Borrow the inner byte slice."] # [inline] pub fn as_bytes (& self) -> & [u8] { self . 0 . as_bytes () } # [doc = " Take ownership of the octet string."] # [inline] pub fn into_bytes (self) -> Box < [u8] > { self . 0 . into_bytes () } # [doc = " Get the length of the inner byte slice."] # [inline] pub fn len (& self) -> Length { self . 0 . len () } # [doc = " Create a [`MessageDigest`] from a [`digest::Digest`]"] # [cfg (feature = "digest")] pub fn from_digest < D > (digest : D) -> der :: Result < Self > where D : digest :: Digest , { Ok (MessageDigest (OctetString :: new (digest . finalize () . to_vec ()) ?)) } # [doc = " Return an [`OctetStringRef`] pointing to the underlying data"] # [inline] pub fn as_octet_string_ref (& self) -> & OctetStringRef { self . 0 . borrow () } }
};
}
