// Generated macro for impl_384 (impl)
macro_rules! Depcrate_typesimpl_384 {
() => {
// Module: crate::types
// Provides: {"impl_384"}
// Dependencies: {}
impl fmt :: Debug for DateFields < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Self { era , era_year , extended_year , month_code , ordinal_month , day , } = * self ; let mut builder = f . debug_struct ("DateFields") ; if let Some (s) = era . and_then (| s | core :: str :: from_utf8 (s) . ok ()) { builder . field ("era" , & Some (s)) ; } else { builder . field ("era" , & era) ; } builder . field ("era_year" , & era_year) ; builder . field ("extended_year" , & extended_year) ; if let Some (s) = month_code . and_then (| s | core :: str :: from_utf8 (s) . ok ()) { builder . field ("month_code" , & Some (s)) ; } else { builder . field ("month_code" , & month_code) ; } builder . field ("ordinal_month" , & ordinal_month) ; builder . field ("day" , & day) ; builder . finish () } }
};
}
