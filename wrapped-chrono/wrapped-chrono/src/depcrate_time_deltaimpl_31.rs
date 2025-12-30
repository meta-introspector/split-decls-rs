// Generated macro for impl_31 (impl)
macro_rules! Depcrate_time_deltaimpl_31 {
() => {
// Module: crate::time_delta
// Provides: {"impl_31"}
// Dependencies: {}
impl core :: iter :: Sum < TimeDelta > for TimeDelta { fn sum < I : Iterator < Item = TimeDelta > > (iter : I) -> TimeDelta { iter . fold (TimeDelta :: zero () , | acc , x | acc + x) } }
};
}
