// Generated macro for impl_259 (impl)
macro_rules! Depcrate_string_drainimpl_259 {
() => {
// Module: crate::string::drain
// Provides: {"impl_259"}
// Dependencies: {}
impl < LenT : LenType > Drop for Drain < '_ , LenT > { fn drop (& mut self) { unsafe { let self_vec = (* self . string) . as_mut_vec () ; let start = self . start . into_usize () ; let end = self . end . into_usize () ; if start <= end && end <= self_vec . len () { self_vec . drain (start .. end) ; } } } }
};
}
