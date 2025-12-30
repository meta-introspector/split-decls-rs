// Generated macro for midnight (function)
macro_rules! Depcrate_chinese_basedmidnight {
() => {
// Module: crate::chinese_based
// Provides: {"midnight"}
// Dependencies: {}
# [doc = " Universal time of midnight at start of a Moment's day at the observation location"] # [doc = ""] # [doc = " Based on functions from _Calendrical Calculations_ by Reingold & Dershowitz."] # [doc = " Lisp reference code: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L5353-L5357"] pub (crate) fn midnight < C : ChineseBased > (moment : Moment) -> Moment { moment - C :: utc_offset (moment . as_rata_die ()) }
};
}
