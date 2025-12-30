// Generated macro for encode_query_cache (function)
macro_rules! Depcrate_persist_saveencode_query_cache {
() => {
// Module: crate::persist::save
// Provides: {"encode_query_cache"}
// Dependencies: {}
fn encode_query_cache (tcx : TyCtxt < '_ > , encoder : FileEncoder) -> FileEncodeResult { tcx . sess . time ("incr_comp_serialize_result_cache" , | | tcx . serialize_query_result_cache (encoder)) }
};
}
