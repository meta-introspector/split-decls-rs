// Generated macro for impl_249 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_249 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_249"}
// Dependencies: {}
impl < D > DiscreteRanged for RangedDate < D > where D : Datelike + TimeValue + Sub < D , Output = Duration > + Add < Duration , Output = D > + Clone , { fn size (& self) -> usize { ((self . 1 . clone () - self . 0 . clone ()) . num_days () . max (- 1) + 1) as usize } fn index_of (& self , value : & D) -> Option < usize > { let ret = (value . clone () - self . 0 . clone ()) . num_days () ; if ret < 0 { return None ; } Some (ret as usize) } fn from_index (& self , index : usize) -> Option < D > { Some (self . 0 . clone () + Duration :: days (index as i64)) } }
};
}
