// Generated macro for impl_348 (impl)
macro_rules! Depcrate_value_fromimpl_348 {
() => {
// Module: crate::value::from
// Provides: {"impl_348"}
// Dependencies: {}
impl < T : Into < Value > , const N : usize > From < [T ; N] > for Value { fn from (array : [T ; N]) -> Self { Value :: Array (array . into_iter () . map (Into :: into) . collect ()) } }
};
}
