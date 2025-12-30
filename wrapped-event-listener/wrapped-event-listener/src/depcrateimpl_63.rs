// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < T > Drop for Event < T > { # [inline] fn drop (& mut self) { self . inner . with_mut (| & mut inner | { if ! inner . is_null () { unsafe { drop (Arc :: from_raw (inner)) ; } } }) } }
};
}
