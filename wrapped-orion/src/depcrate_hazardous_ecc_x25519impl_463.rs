// Generated macro for impl_463 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_463 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_463"}
// Dependencies: {}
impl PartialEq for FieldElement { fn eq (& self , other : & Self) -> bool { use subtle :: ConstantTimeEq ; self . as_bytes () . ct_eq (& other . as_bytes ()) . into () } }
};
}
