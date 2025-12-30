// Generated macro for impl_1132 (impl)
macro_rules! Depcrate_tz_zicimpl_1132 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1132"}
// Dependencies: {}
impl RuleOnP { # [doc = " Given a year and a month, return the specific date for this \"day of the"] # [doc = " month\" specification."] fn date (& self , year : t :: Year , month : t :: Month) -> Result < Date , Error > { match * self { RuleOnP :: Day { day } => Date :: new_ranged (year , month , day) , RuleOnP :: Last { weekday } => { let date = Date :: new_ranged (year , month , C (1) . rinto ()) . unwrap () ; date . nth_weekday_of_month (- 1 , weekday) } RuleOnP :: OnOrBefore { weekday , day } => { let start = Date :: new_ranged (year , month , day) ? . checked_add (1 . day ()) ? ; start . nth_weekday (- 1 , weekday) } RuleOnP :: OnOrAfter { weekday , day } => { let start = Date :: new_ranged (year , month , day) ? . checked_sub (1 . day ()) ? ; start . nth_weekday (1 , weekday) } } } }
};
}
