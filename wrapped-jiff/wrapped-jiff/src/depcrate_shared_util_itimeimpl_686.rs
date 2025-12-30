// Generated macro for impl_686 (impl)
macro_rules! Depcrate_shared_util_itimeimpl_686 {
() => {
// Module: crate::shared::util::itime
// Provides: {"impl_686"}
// Dependencies: {}
impl ITimeNanosecond { # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) const fn to_time (& self) -> ITime { let mut nanosecond = self . nanosecond ; let mut time = ITime :: ZERO ; if nanosecond != 0 { time . hour = (nanosecond / 3_600_000_000_000) as i8 ; nanosecond %= 3_600_000_000_000 ; if nanosecond != 0 { time . minute = (nanosecond / 60_000_000_000) as i8 ; nanosecond %= 60_000_000_000 ; if nanosecond != 0 { time . second = (nanosecond / 1_000_000_000) as i8 ; time . subsec_nanosecond = (nanosecond % 1_000_000_000) as i32 ; } } } time } }
};
}
