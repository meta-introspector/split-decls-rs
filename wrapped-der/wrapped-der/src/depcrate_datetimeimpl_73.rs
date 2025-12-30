// Generated macro for impl_73 (impl)
macro_rules! Depcrate_datetimeimpl_73 {
() => {
// Module: crate::datetime
// Provides: {"impl_73"}
// Dependencies: {}
impl FromStr for DateTime { type Err = Error ; fn from_str (s : & str) -> Result < Self > { match * s . as_bytes () { [year1 , year2 , year3 , year4 , b'-' , month1 , month2 , b'-' , day1 , day2 , b'T' , hour1 , hour2 , b':' , min1 , min2 , b':' , sec1 , sec2 , b'Z' ,] => { let tag = Tag :: GeneralizedTime ; let year = decode_year (& [year1 , year2 , year3 , year4]) ? ; let month = decode_decimal (tag , month1 , month2) . map_err (| _ | ErrorKind :: DateTime) ? ; let day = decode_decimal (tag , day1 , day2) . map_err (| _ | ErrorKind :: DateTime) ? ; let hour = decode_decimal (tag , hour1 , hour2) . map_err (| _ | ErrorKind :: DateTime) ? ; let minutes = decode_decimal (tag , min1 , min2) . map_err (| _ | ErrorKind :: DateTime) ? ; let seconds = decode_decimal (tag , sec1 , sec2) . map_err (| _ | ErrorKind :: DateTime) ? ; Self :: new (year , month , day , hour , minutes , seconds) } _ => Err (ErrorKind :: DateTime . into ()) , } } }
};
}
