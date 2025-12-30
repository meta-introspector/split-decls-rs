// Generated macro for impl_155 (impl)
macro_rules! Depcrate_arbitrary__core_iterimpl_155 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"impl_155"}
// Dependencies: {}
impl < T : 'static + Clone , A : fmt :: Debug + 'static + Iterator < Item = & 'static T > , > functor :: ArbitraryF1 < A > for Cloned < A > { type Parameters = () ; fn lift1_with < S > (base : S , _args : Self :: Parameters) -> BoxedStrategy < Self > where S : Strategy < Value = A > + 'static , { base . prop_map (Iterator :: cloned) . boxed () } }
};
}
