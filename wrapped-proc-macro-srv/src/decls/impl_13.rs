macro_rules! deps {
    () => {
        ProcMacroSrvSpan!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl ProcMacroSrvSpan for SpanId { type Server = server_impl :: token_id :: SpanIdServer ; fn make_server (call_site : Self , def_site : Self , mixed_site : Self) -> Self :: Server { Self :: Server { call_site , def_site , mixed_site } } }
    };
}

impl_13!()