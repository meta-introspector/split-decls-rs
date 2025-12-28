macro_rules! deps {
    () => {
        Key!();
        Remote!();
        Section!();
    };
}

macro_rules! impl_723 {
    () => {
        deps!();
        impl Section for Remote { fn name (& self) -> & str { "remote" } fn keys (& self) -> & [& dyn Key] { & [& Self :: PUSH_DEFAULT , & Self :: TAG_OPT , & Self :: URL , & Self :: PUSH_URL , & Self :: FETCH , & Self :: PUSH , & Self :: PROXY , & Self :: PROXY_AUTH_METHOD ,] } }
    };
}

impl_723!()