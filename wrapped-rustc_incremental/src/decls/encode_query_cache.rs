macro_rules! encode_query_cache {
    () => {
        fn encode_query_cache (tcx : TyCtxt < '_ > , encoder : FileEncoder) -> FileEncodeResult { tcx . sess . time ("incr_comp_serialize_result_cache" , | | tcx . serialize_query_result_cache (encoder)) }
    };
}

encode_query_cache!();