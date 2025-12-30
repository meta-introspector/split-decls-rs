// Generated macro for impl_606 (impl)
macro_rules! Depcrate_shared_posiximpl_606 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_606"}
// Dependencies: {}
impl PosixDayTime { # [doc = " Turns this POSIX datetime spec into a civil datetime in the year given"] # [doc = " with the given offset. The datetimes returned are offset by the given"] # [doc = " offset. For wall clock time, an offset of `0` should be given. For"] # [doc = " UTC time, the offset (standard or DST) corresponding to this time"] # [doc = " spec should be given."] # [doc = ""] # [doc = " The datetime returned is guaranteed to have a year component equal"] # [doc = " to the year given. This guarantee is upheld even when the datetime"] # [doc = " specification (combined with the offset) would extend past the end of"] # [doc = " the year (or before the start of the year). In this case, the maximal"] # [doc = " (or minimal) datetime for the given year is returned."] pub (crate) fn to_datetime (& self , year : i16 , offset : IOffset) -> IDateTime { let mkmin = | | IDateTime { date : IDate { year , month : 1 , day : 1 } , time : ITime :: MIN , } ; let mkmax = | | IDateTime { date : IDate { year , month : 12 , day : 31 } , time : ITime :: MAX , } ; let Some (date) = self . date . to_date (year) else { return mkmax () } ; let offset = self . time . second - offset . second ; let days = offset . div_euclid (86400) ; let second = offset . rem_euclid (86400) ; let Ok (date) = date . checked_add_days (days) else { return if offset < 0 { mkmin () } else { mkmax () } ; } ; if date . year < year { mkmin () } else if date . year > year { mkmax () } else { let time = ITimeSecond { second } . to_time () ; IDateTime { date , time } } } }
};
}
