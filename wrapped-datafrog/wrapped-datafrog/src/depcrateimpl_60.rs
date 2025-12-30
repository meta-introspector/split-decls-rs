// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'tuple , Tuple : 'tuple + Copy + Ord > FromIterator < & 'tuple Tuple > for Relation < Tuple > { fn from_iter < I > (iterator : I) -> Self where I : IntoIterator < Item = & 'tuple Tuple > , { Relation :: from_vec (iterator . into_iter () . cloned () . collect ()) } }
};
}
