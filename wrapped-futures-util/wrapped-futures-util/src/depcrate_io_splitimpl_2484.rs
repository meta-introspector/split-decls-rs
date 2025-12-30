// Generated macro for impl_2484 (impl)
macro_rules! Depcrate_io_splitimpl_2484 {
() => {
// Module: crate::io::split
// Provides: {"impl_2484"}
// Dependencies: {}
impl < T > WriteHalf < T > { # [doc = " Checks if this `WriteHalf` and some `ReadHalf` were split from the same stream."] pub fn is_pair_of (& self , other : & ReadHalf < T >) -> bool { self . handle . is_pair_of (& other . handle) } }
};
}
