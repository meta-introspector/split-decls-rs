// Generated macro for impl_1785 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1785 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1785"}
// Dependencies: {}
impl < Fut > FromIterator < Fut > for FuturesUnordered < Fut > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Fut > , { let acc = Self :: new () ; iter . into_iter () . fold (acc , | acc , item | { acc . push (item) ; acc }) } }
};
}
