// Generated macro for impl_30 (impl)
macro_rules! Depcrate_dateimpl_30 {
() => {
// Module: crate::date
// Provides: {"impl_30"}
// Dependencies: {}
impl < C : IntoAnyCalendar > Date < C > { # [doc = " Type-erase the date, converting it to a date for [`AnyCalendar`]"] pub fn to_any (self) -> Date < AnyCalendar > { Date :: from_raw (self . calendar . date_to_any (& self . inner) , self . calendar . to_any () ,) } }
};
}
