// Generated macro for impl_271 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_271 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_271"}
// Dependencies: {}
impl < DT > ReversibleRanged for RangedDateTime < DT > where DT : Datelike + Timelike + TimeValue + Clone + PartialOrd , DT : Add < Duration , Output = DT > , DT : Sub < DT , Output = Duration > , RangedDate < DT :: DateType > : Ranged < ValueType = DT :: DateType > , { # [doc = " Perform the reverse mapping"] fn unmap (& self , input : i32 , limit : (i32 , i32)) -> Option < Self :: ValueType > { Some (TimeValue :: unmap_coord (input , & self . 0 , & self . 1 , limit)) } }
};
}
