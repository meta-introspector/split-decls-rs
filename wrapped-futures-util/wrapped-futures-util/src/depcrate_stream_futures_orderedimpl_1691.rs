// Generated macro for impl_1691 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1691 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1691"}
// Dependencies: {}
impl < Fut : Future > Extend < Fut > for FuturesOrdered < Fut > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Fut > , { for item in iter { self . push_back (item) ; } } }
};
}
