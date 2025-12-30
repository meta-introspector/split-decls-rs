// Generated macro for impl_15 (impl)
macro_rules! Depcrate_hasherimpl_15 {
() => {
// Module: crate::hasher
// Provides: {"impl_15"}
// Dependencies: {}
impl Hasher for AddressHasher { # [inline] fn finish (& self) -> u64 { self . state } # [inline] fn write (& mut self , bytes : & [u8]) { debug_assert_eq ! (bytes . len () , ADDRESS_BYTES , "This hasher is intended to be used with addresses and nothing else") ; let chunk : & [u8 ; mem :: size_of :: < u64 > ()] = bytes [self . offset .. self . offset + mem :: size_of :: < u64 > ()] . try_into () . unwrap () ; self . state = u64 :: from_ne_bytes (* chunk) ; } }
};
}
