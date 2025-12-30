// Generated macro for impl_76 (impl)
macro_rules! Depcrate_dateimpl_76 {
() => {
// Module: crate::date
// Provides: {"impl_76"}
// Dependencies: {}
impl < Tz : TimeZone > fmt :: Display for Date < Tz > where Tz :: Offset : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . naive_local () . fmt (f) ? ; self . offset . fmt (f) } }
};
}
