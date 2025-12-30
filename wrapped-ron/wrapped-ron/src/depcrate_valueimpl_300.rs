// Generated macro for impl_300 (impl)
macro_rules! Depcrate_valueimpl_300 {
() => {
// Module: crate::value
// Provides: {"impl_300"}
// Dependencies: {}
impl < T : Into < Value > > From < Vec < T > > for Value { fn from (value : Vec < T >) -> Self { value . into_iter () . collect () } }
};
}
