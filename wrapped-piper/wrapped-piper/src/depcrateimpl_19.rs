// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Drop for Pipe { fn drop (& mut self) { unsafe { Vec :: from_raw_parts (self . buffer , 0 , self . cap) ; } } }
};
}
