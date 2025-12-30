// Generated macro for impl_1787 (impl)
macro_rules! Depcrate_stream_futures_unorderedimpl_1787 {
() => {
// Module: crate::stream::futures_unordered
// Provides: {"impl_1787"}
// Dependencies: {}
impl < Fut > Extend < Fut > for FuturesUnordered < Fut > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Fut > , { for item in iter { self . push (item) ; } } }
};
}
