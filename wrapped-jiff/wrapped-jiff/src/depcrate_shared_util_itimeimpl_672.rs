// Generated macro for impl_672 (impl)
macro_rules! Depcrate_shared_util_itimeimpl_672 {
() => {
// Module: crate::shared::util::itime
// Provides: {"impl_672"}
// Dependencies: {}
impl ITimestamp { const MIN : ITimestamp = ITimestamp { second : - 377705023201 , nanosecond : 0 } ; const MAX : ITimestamp = ITimestamp { second : 253402207200 , nanosecond : 999_999_999 } ; # [doc = " Creates an `ITimestamp` from a Unix timestamp in seconds."] # [inline] pub (crate) const fn from_second (second : i64) -> ITimestamp { ITimestamp { second , nanosecond : 0 } } # [doc = " Converts a Unix timestamp with an offset to a Gregorian datetime."] # [doc = ""] # [doc = " The offset should correspond to the number of seconds required to"] # [doc = " add to this timestamp to get the local time."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) const fn to_datetime (& self , offset : IOffset) -> IDateTime { let ITimestamp { mut second , mut nanosecond } = * self ; second += offset . second as i64 ; let mut epoch_day = second . div_euclid (86_400) as i32 ; second = second . rem_euclid (86_400) ; if nanosecond < 0 { if second > 0 { second -= 1 ; nanosecond += 1_000_000_000 ; } else { epoch_day -= 1 ; second += 86_399 ; nanosecond += 1_000_000_000 ; } } let date = IEpochDay { epoch_day } . to_date () ; let mut time = ITimeSecond { second : second as i32 } . to_time () ; time . subsec_nanosecond = nanosecond ; IDateTime { date , time } } }
};
}
