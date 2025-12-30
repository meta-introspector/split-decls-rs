// Generated macro for impl_67 (impl)
macro_rules! Depcrate_dateimpl_67 {
() => {
// Module: crate::date
// Provides: {"impl_67"}
// Dependencies: {}
impl < Tz : TimeZone > PartialOrd for Date < Tz > { fn partial_cmp (& self , other : & Date < Tz >) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
