// Generated macro for impl_727 (impl)
macro_rules! Depcrate_signed_durationimpl_727 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_727"}
// Dependencies: {}
impl < 'a > core :: iter :: Sum < & 'a Self > for SignedDuration { fn sum < I : Iterator < Item = & 'a Self > > (iter : I) -> Self { iter . fold (Self :: new (0 , 0) , | acc , d | acc + * d) } }
};
}
