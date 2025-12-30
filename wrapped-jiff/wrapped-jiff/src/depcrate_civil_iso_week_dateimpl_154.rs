// Generated macro for impl_154 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_154 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_154"}
// Dependencies: {}
impl Ord for ISOWeekDate { # [inline] fn cmp (& self , other : & ISOWeekDate) -> core :: cmp :: Ordering { (self . year . get () , self . week . get () , self . weekday . to_monday_one_offset ()) . cmp (& (other . year . get () , other . week . get () , other . weekday . to_monday_one_offset () ,)) } }
};
}
