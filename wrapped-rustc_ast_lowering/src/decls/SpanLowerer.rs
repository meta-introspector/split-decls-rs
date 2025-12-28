macro_rules! SpanLowerer {
    () => {
        struct SpanLowerer { is_incremental : bool , def_id : LocalDefId , }
    };
}

SpanLowerer!()