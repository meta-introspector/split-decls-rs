// Generated macro for impl_2482 (impl)
macro_rules! Depcrate_io_splitimpl_2482 {
() => {
// Module: crate::io::split
// Provides: {"impl_2482"}
// Dependencies: {}
impl < T > ReadHalf < T > { # [doc = " Checks if this `ReadHalf` and some `WriteHalf` were split from the same stream."] pub fn is_pair_of (& self , other : & WriteHalf < T >) -> bool { self . handle . is_pair_of (& other . handle) } }
};
}
