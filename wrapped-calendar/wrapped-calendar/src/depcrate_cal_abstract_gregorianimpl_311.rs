// Generated macro for impl_311 (impl)
macro_rules! Depcrate_cal_abstract_gregorianimpl_311 {
() => {
// Module: crate::cal::abstract_gregorian
// Provides: {"impl_311"}
// Dependencies: {}
impl ArithmeticDate < AbstractGregorian < IsoEra > > { pub (crate) fn new_gregorian < Y : GregorianYears > (year : i32 , month : u8 , day : u8 ,) -> Result < Self , RangeError > { ArithmeticDate :: try_from_ymd (year + Y :: EXTENDED_YEAR_OFFSET , month , day) . map_err (| e | { if e . field == "year" { RangeError { value : e . value - Y :: EXTENDED_YEAR_OFFSET , min : e . min - Y :: EXTENDED_YEAR_OFFSET , max : e . max - Y :: EXTENDED_YEAR_OFFSET , .. e } } else { e } }) } }
};
}
