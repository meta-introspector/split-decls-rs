// Generated macro for impl_352 (impl)
macro_rules! Depcrate_coord_ranged1d_discreteimpl_352 {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"impl_352"}
// Dependencies: {}
impl < T , D : DiscreteRanged + Ranged < ValueType = T > > ValueFormatter < SegmentValue < T > > for SegmentedCoord < D > where D : ValueFormatter < T > , { fn format (value : & SegmentValue < T >) -> String { match value { SegmentValue :: Exact (ref value) => D :: format (value) , SegmentValue :: CenterOf (ref value) => D :: format (value) , _ => "" . to_string () , } } }
};
}
