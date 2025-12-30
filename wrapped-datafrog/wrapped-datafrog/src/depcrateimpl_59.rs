// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < Tuple : Ord > FromIterator < Tuple > for Relation < Tuple > { fn from_iter < I > (iterator : I) -> Self where I : IntoIterator < Item = Tuple > , { Relation :: from_vec (iterator . into_iter () . collect ()) } }
};
}
