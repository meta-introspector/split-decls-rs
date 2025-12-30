// Generated macro for impl_37 (impl)
macro_rules! Depcrate_dateimpl_37 {
() => {
// Module: crate::date
// Provides: {"impl_37"}
// Dependencies: {}
impl < A : AsCalendar + Clone > Clone for Date < A > { fn clone (& self) -> Self { Self { inner : self . inner , calendar : self . calendar . clone () , } } }
};
}
