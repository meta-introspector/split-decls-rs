// Generated macro for impl_30 (impl)
macro_rules! Depcrate_time_deltaimpl_30 {
() => {
// Module: crate::time_delta
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > core :: iter :: Sum < & 'a TimeDelta > for TimeDelta { fn sum < I : Iterator < Item = & 'a TimeDelta > > (iter : I) -> TimeDelta { iter . fold (TimeDelta :: zero () , | acc , x | acc + * x) } }
};
}
