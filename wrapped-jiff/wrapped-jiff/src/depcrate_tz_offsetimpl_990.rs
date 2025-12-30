// Generated macro for impl_990 (impl)
macro_rules! Depcrate_tz_offsetimpl_990 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_990"}
// Dependencies: {}
impl core :: fmt :: Debug for Offset { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let sign = if self . seconds_ranged () < C (0) { "-" } else { "" } ; write ! (f , "{sign}{:02}:{:02}:{:02}" , self . part_hours_ranged () . abs () , self . part_minutes_ranged () . abs () , self . part_seconds_ranged () . abs () ,) } }
};
}
