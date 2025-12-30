// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_tz_zicimpl_1130 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1130"}
// Dependencies: {}
impl FromStr for RuleInP { type Err = Error ; fn from_str (field : & str) -> Result < RuleInP , Error > { static MONTH_PREFIXES : & [(u8 , & str , & str)] = & [(1 , "January" , "Ja") , (2 , "February" , "F") , (3 , "March" , "Mar") , (4 , "April" , "Ap") , (5 , "May" , "May") , (6 , "June" , "Jun") , (7 , "July" , "Jul") , (8 , "August" , "Au") , (9 , "September" , "S") , (10 , "October" , "O") , (11 , "November" , "N") , (12 , "December" , "D") ,] ; for & (number , name , prefix) in MONTH_PREFIXES { if field . starts_with (prefix) && name . starts_with (field) { let month = t :: Month :: new (number) . unwrap () ; return Ok (RuleInP { month }) ; } } Err (err ! ("unrecognized month name: {field:?}")) } }
};
}
