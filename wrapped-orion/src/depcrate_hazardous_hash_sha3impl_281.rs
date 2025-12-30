// Generated macro for impl_281 (impl)
macro_rules! Depcrate_hazardous_hash_sha3impl_281 {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"impl_281"}
// Dependencies: {}
impl < const RATE : usize > Drop for Sha3 < RATE > { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; self . buffer . iter_mut () . zeroize () ; self . leftover . zeroize () ; } }
};
}
