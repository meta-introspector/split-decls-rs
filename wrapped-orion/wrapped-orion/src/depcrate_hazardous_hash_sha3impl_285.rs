// Generated macro for impl_285 (impl)
macro_rules! Depcrate_hazardous_hash_sha3impl_285 {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"impl_285"}
// Dependencies: {}
impl < const RATE : usize > Drop for Shake < RATE > { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; self . buffer . iter_mut () . zeroize () ; self . until_absorb . zeroize () ; self . to_squeeze . zeroize () ; } }
};
}
