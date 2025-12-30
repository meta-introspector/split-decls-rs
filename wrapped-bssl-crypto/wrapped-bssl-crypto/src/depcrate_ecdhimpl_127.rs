// Generated macro for impl_127 (impl)
macro_rules! Depcrate_ecdhimpl_127 {
() => {
// Module: crate::ecdh
// Provides: {"impl_127"}
// Dependencies: {}
impl < C : ec :: Curve > PublicKey < C > { # [doc = " Parse a public key in uncompressed X9.62 format. (This is the common"] # [doc = " format for elliptic curve points beginning with an 0x04 byte.)"] pub fn from_x962_uncompressed (x962 : & [u8]) -> Option < Self > { let point = ec :: Point :: from_x962_uncompressed (C :: group (sealed :: Sealed) , x962) ? ; Some (Self { point , marker : PhantomData , }) } # [doc = " Serialize this key as uncompressed X9.62 format."] pub fn to_x962_uncompressed (& self) -> Buffer { self . point . to_x962_uncompressed () } }
};
}
