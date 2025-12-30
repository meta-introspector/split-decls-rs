// Generated macro for impl_782 (impl)
macro_rules! Depcrate_odbimpl_782 {
() => {
// Module: crate::odb
// Provides: {"impl_782"}
// Dependencies: {}
impl < 'repo > Drop for OdbPackwriter < 'repo > { fn drop (& mut self) { unsafe { let writepack = & * self . raw ; match writepack . free { Some (free) => free (self . raw) , None => () , } ; drop (Box :: from_raw (self . progress_payload_ptr)) ; } } }
};
}
