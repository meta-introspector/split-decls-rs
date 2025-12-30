// Generated macro for impl_32 (impl)
macro_rules! Depcrate_civil_dateimpl_32 {
() => {
// Module: crate::civil::date
// Provides: {"impl_32"}
// Dependencies: {}
impl core :: fmt :: Display for Date { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: fmt :: StdFmtWrite ; DEFAULT_DATETIME_PRINTER . print_date (self , StdFmtWrite (f)) . map_err (| _ | core :: fmt :: Error) } }
};
}
