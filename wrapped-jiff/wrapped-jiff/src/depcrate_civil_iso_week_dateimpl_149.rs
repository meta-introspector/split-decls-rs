// Generated macro for impl_149 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_149 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_149"}
// Dependencies: {}
impl core :: fmt :: Debug for ISOWeekDate { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("ISOWeekDate") . field ("year" , & self . year_ranged () . debug ()) . field ("week" , & self . week_ranged () . debug ()) . field ("weekday" , & self . weekday) . finish () } }
};
}
