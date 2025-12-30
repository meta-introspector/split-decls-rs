// Generated macro for impl_134 (impl)
macro_rules! Depcrate_rawimpl_134 {
() => {
// Module: crate::raw
// Provides: {"impl_134"}
// Dependencies: {}
impl < T > Default for RawIterHash < T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : unsafe { RawIterHashInner :: new (& RawTableInner :: NEW , 0) } , _marker : PhantomData , } } }
};
}
