// Generated macro for impl_239 (impl)
macro_rules! Depcrate_typesimpl_239 {
() => {
// Module: crate::types
// Provides: {"impl_239"}
// Dependencies: {}
impl From < std :: time :: Duration > for Timespec { fn from (value : std :: time :: Duration) -> Self { Timespec :: new () . sec (value . as_secs ()) . nsec (value . subsec_nanos ()) } }
};
}
