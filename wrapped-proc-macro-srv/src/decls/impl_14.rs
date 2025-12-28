macro_rules! deps {
    () => {
        ProcMacroSrvSpan!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl ProcMacroSrvSpan for Span { type Server = server_impl :: rust_analyzer_span :: RaSpanServer ; fn make_server (call_site : Self , def_site : Self , mixed_site : Self) -> Self :: Server { Self :: Server { call_site , def_site , mixed_site , tracked_env_vars : Default :: default () , tracked_paths : Default :: default () , } } }
    };
}

impl_14!()