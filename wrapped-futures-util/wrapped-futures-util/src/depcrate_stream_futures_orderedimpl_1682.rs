// Generated macro for impl_1682 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1682 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1682"}
// Dependencies: {}
impl < T > Future for OrderWrapper < T > where T : Future , { type Output = OrderWrapper < T :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let index = self . index ; self . project () . data . poll (cx) . map (| output | OrderWrapper { data : output , index }) } }
};
}
