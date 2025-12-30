// Generated macro for impl_242 (impl)
macro_rules! Depcrate_frameimpl_242 {
() => {
// Module: crate::frame
// Provides: {"impl_242"}
// Dependencies: {}
impl TryFrom < QFrame > for EnrichedHeaders { type Error = BoxError ; fn try_from (value : QFrame) -> Result < Self , Self :: Error > { match value { QFrame :: Headers { header_block } => { let mut qpack_decoder = quiche :: h3 :: qpack :: Decoder :: new () ; let headers = qpack_decoder . decode (& header_block , u64 :: MAX) . unwrap () ; Ok (EnrichedHeaders :: from (headers)) } , _ => Err ("Cannot convert non-Headers frame into HeadersFrame" . into ()) , } } }
};
}
