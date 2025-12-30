// Generated macro for impl_62 (impl)
macro_rules! Depcrate_private_keyimpl_62 {
() => {
// Module: crate::private_key
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl < 'a > DecodeValue < 'a > for OtherPrimeInfos < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Err (reader . error (der :: ErrorKind :: Value { tag : Tag :: Integer })) } }
};
}
