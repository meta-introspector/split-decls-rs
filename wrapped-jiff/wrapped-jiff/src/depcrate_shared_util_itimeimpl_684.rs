// Generated macro for impl_684 (impl)
macro_rules! Depcrate_shared_util_itimeimpl_684 {
() => {
// Module: crate::shared::util::itime
// Provides: {"impl_684"}
// Dependencies: {}
impl ITimeSecond { # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) const fn to_time (& self) -> ITime { let mut second = self . second ; let mut time = ITime :: ZERO ; if second != 0 { time . hour = (second / 3600) as i8 ; second %= 3600 ; if second != 0 { time . minute = (second / 60) as i8 ; time . second = (second % 60) as i8 ; } } time } }
};
}
