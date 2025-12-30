// Generated macro for impl_1689 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1689 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1689"}
// Dependencies: {}
impl < Fut : Future > FromIterator < Fut > for FuturesOrdered < Fut > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Fut > , { let acc = Self :: new () ; iter . into_iter () . fold (acc , | mut acc , item | { acc . push_back (item) ; acc }) } }
};
}
