// Generated macro for impl_150 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_150 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_150"}
// Dependencies: {}
impl core :: fmt :: Display for ISOWeekDate { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; DEFAULT_DATETIME_PRINTER . print_iso_week_date (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } }
};
}
