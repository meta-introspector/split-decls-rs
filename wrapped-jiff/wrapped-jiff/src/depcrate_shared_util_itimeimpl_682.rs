// Generated macro for impl_682 (impl)
macro_rules! Depcrate_shared_util_itimeimpl_682 {
() => {
// Module: crate::shared::util::itime
// Provides: {"impl_682"}
// Dependencies: {}
impl ITime { pub (crate) const ZERO : ITime = ITime { hour : 0 , minute : 0 , second : 0 , subsec_nanosecond : 0 } ; pub (crate) const MIN : ITime = ITime { hour : 0 , minute : 0 , second : 0 , subsec_nanosecond : 0 } ; pub (crate) const MAX : ITime = ITime { hour : 23 , minute : 59 , second : 59 , subsec_nanosecond : 999_999_999 , } ; # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) const fn to_second (& self) -> ITimeSecond { let mut second : i32 = 0 ; second += (self . hour as i32) * 3600 ; second += (self . minute as i32) * 60 ; second += self . second as i32 ; ITimeSecond { second } } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) const fn to_nanosecond (& self) -> ITimeNanosecond { let mut nanosecond : i64 = 0 ; nanosecond += (self . hour as i64) * 3_600_000_000_000 ; nanosecond += (self . minute as i64) * 60_000_000_000 ; nanosecond += (self . second as i64) * 1_000_000_000 ; nanosecond += self . subsec_nanosecond as i64 ; ITimeNanosecond { nanosecond } } }
};
}
