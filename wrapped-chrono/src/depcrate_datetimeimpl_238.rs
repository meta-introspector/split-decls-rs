// Generated macro for impl_238 (impl)
macro_rules! Depcrate_datetimeimpl_238 {
() => {
// Module: crate::datetime
// Provides: {"impl_238"}
// Dependencies: {}
impl < Tz : TimeZone > fmt :: Display for DateTime < Tz > where Tz :: Offset : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . overflowing_naive_local () . fmt (f) ? ; f . write_char (' ') ? ; self . offset . fmt (f) } }
};
}
