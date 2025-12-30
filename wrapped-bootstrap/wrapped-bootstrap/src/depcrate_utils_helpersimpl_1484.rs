// Generated macro for impl_1484 (impl)
macro_rules! Depcrate_utils_helpersimpl_1484 {
() => {
// Module: crate::utils::helpers
// Provides: {"impl_1484"}
// Dependencies: {}
impl Drop for TimeIt { fn drop (& mut self) { let time = self . 1 . elapsed () ; if ! self . 0 { println ! ("\tfinished in {}.{:03} seconds" , time . as_secs () , time . subsec_millis ()) ; } } }
};
}
