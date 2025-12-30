// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1081 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1081"}
// Dependencies: {}
impl < St : Stream + Unpin > Future for PollStreamFut < St > { type Output = Option < (St :: Item , Self) > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut stream = self . project () . stream ; let item = if let Some (stream) = stream . as_mut () . as_pin_mut () { ready ! (stream . poll_next (cx)) } else { None } ; let next_item_fut = Self :: new (stream . get_mut () . take ()) ; let out = item . map (| item | (item , next_item_fut)) ; Poll :: Ready (out) } }
};
}
