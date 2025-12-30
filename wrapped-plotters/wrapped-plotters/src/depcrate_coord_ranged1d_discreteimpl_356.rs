// Generated macro for impl_356 (impl)
macro_rules! Depcrate_coord_ranged1d_discreteimpl_356 {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"impl_356"}
// Dependencies: {}
impl < DC : DiscreteRanged > ReversibleRanged for DC { fn unmap (& self , input : i32 , limit : (i32 , i32)) -> Option < Self :: ValueType > { let idx = (f64 :: from (input - limit . 0) * (self . size () as f64) / f64 :: from (limit . 1 - limit . 0)) . floor () as usize ; self . from_index (idx) } }
};
}
