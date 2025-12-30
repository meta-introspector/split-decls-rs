// Generated macro for impl_393 (impl)
macro_rules! Depcrate_typesimpl_393 {
() => {
// Module: crate::types
// Provides: {"impl_393"}
// Dependencies: {}
impl MonthCode { # [doc = " Returns an option which is `Some` containing the non-month version of a leap month"] # [doc = " if the [`MonthCode`] this method is called upon is a leap month, and `None` otherwise."] # [doc = " This method assumes the [`MonthCode`] is valid."] # [deprecated (since = "2.1.0")] pub fn get_normal_if_leap (self) -> Option < MonthCode > { let bytes = self . 0 . all_bytes () ; if bytes [3] == b'L' { Some (MonthCode (TinyAsciiStr :: try_from_utf8 (& bytes [0 .. 3]) . ok () ?)) } else { None } } # [deprecated (since = "2.1.0")] # [doc = " Get the month number and whether or not it is leap from the month code"] pub fn parsed (self) -> Option < (u8 , bool) > { Month :: try_from_utf8 (self . 0 . as_bytes ()) . ok () . map (| m | (m . number () , m . is_leap ())) } # [doc = " Deprecated, use `Month::new(m).code()`"] # [deprecated (since = "2.2.0" , note = "use `Month::new(m).code()`")] pub fn new_normal (number : u8) -> Option < Self > { (1 ..= 99) . contains (& number) . then (| | Month :: new_unchecked (number , LeapStatus :: Normal) . code ()) } # [doc = " Deprecated, use `Month::leap(m).code()`"] # [deprecated (since = "2.2.0" , note = "use `Month::leap(m).code()`")] pub fn new_leap (number : u8) -> Option < Self > { (1 ..= 99) . contains (& number) . then (| | Month :: new_unchecked (number , LeapStatus :: Leap) . code ()) } }
};
}
