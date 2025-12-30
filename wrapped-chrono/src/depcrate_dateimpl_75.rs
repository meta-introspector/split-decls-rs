// Generated macro for impl_75 (impl)
macro_rules! Depcrate_dateimpl_75 {
() => {
// Module: crate::date
// Provides: {"impl_75"}
// Dependencies: {}
impl < Tz : TimeZone > fmt :: Debug for Date < Tz > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
};
}
