// Generated macro for impl_476 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_476 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_476"}
// Dependencies: {}
impl PartialEq < & [u8] > for PublicKey { fn eq (& self , other : & & [u8]) -> bool { if other . len () != PUBLIC_KEY_SIZE { return false ; } let other : [u8 ; 32] = (* other) . try_into () . unwrap () ; self . fe == FieldElement :: from_bytes (& other) } }
};
}
