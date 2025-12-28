macro_rules! deps {
    () => {
        MetaOnlyCallsite!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl tracing_core :: callsite :: Callsite for MetaOnlyCallsite { fn set_interest (& self , _ : tracing_core :: subscriber :: Interest) { } fn metadata (& self) -> & Metadata < '_ > { self . 0 } }
    };
}

impl_7!()