// Generated macro for impl_63 (impl)
macro_rules! Depcrate_tableimpl_63 {
() => {
// Module: crate::table
// Provides: {"impl_63"}
// Dependencies: {}
impl RuleInfo { # [doc = " Returns whether this rule is in effect during the given year."] pub fn applies_to_year (& self , year : i64) -> bool { use line :: Year :: * ; match (self . from_year , self . to_year) { (Number (from) , None) => year == from , (Number (from) , Some (Maximum)) => year >= from , (Number (from) , Some (Number (to))) => year >= from && year <= to , _ => unreachable ! () , } } pub fn absolute_datetime (& self , year : i64 , utc_offset : i64 , dst_offset : i64) -> i64 { let offset = match self . time_type { TimeType :: UTC => 0 , TimeType :: Standard => utc_offset , TimeType :: Wall => utc_offset + dst_offset , } ; let changetime = ChangeTime :: UntilDay (Year :: Number (year) , self . month , self . day) ; let unused = 0 ; changetime . to_timestamp (unused , unused) + self . time - offset } }
};
}
