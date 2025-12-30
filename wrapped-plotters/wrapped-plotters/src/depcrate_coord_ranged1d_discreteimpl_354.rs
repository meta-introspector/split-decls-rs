// Generated macro for impl_354 (impl)
macro_rules! Depcrate_coord_ranged1d_discreteimpl_354 {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"impl_354"}
// Dependencies: {}
impl < D : DiscreteRanged > DiscreteRanged for SegmentedCoord < D > { fn size (& self) -> usize { self . 0 . size () + 1 } fn index_of (& self , value : & Self :: ValueType) -> Option < usize > { match value { SegmentValue :: Exact (value) => self . 0 . index_of (value) , SegmentValue :: CenterOf (value) => self . 0 . index_of (value) , SegmentValue :: Last => Some (self . 0 . size ()) , } } fn from_index (& self , idx : usize) -> Option < Self :: ValueType > { match idx { idx if idx < self . 0 . size () => self . 0 . from_index (idx) . map (SegmentValue :: Exact) , idx if idx == self . 0 . size () => Some (SegmentValue :: Last) , _ => None , } } }
};
}
