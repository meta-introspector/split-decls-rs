// Generated macro for impl_726 (impl)
macro_rules! Depcrate_signed_durationimpl_726 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_726"}
// Dependencies: {}
impl core :: iter :: Sum for SignedDuration { fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { iter . fold (Self :: new (0 , 0) , | acc , d | acc + d) } }
};
}
