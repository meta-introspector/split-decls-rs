// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl Drop for Acquired { fn drop (& mut self) { if ! self . disabled { drop (self . client . release (Some (& self . data))) ; } } }
};
}
