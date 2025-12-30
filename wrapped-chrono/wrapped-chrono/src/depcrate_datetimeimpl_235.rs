// Generated macro for impl_235 (impl)
macro_rules! Depcrate_datetimeimpl_235 {
() => {
// Module: crate::datetime
// Provides: {"impl_235"}
// Dependencies: {}
impl < Tz : TimeZone > fmt :: Debug for DateTime < Tz > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . overflowing_naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
};
}
