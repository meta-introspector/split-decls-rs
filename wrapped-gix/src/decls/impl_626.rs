macro_rules! deps {
    () => {
        Credential!();
        Key!();
        Section!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl Section for Credential { fn name (& self) -> & str { "credential" } fn keys (& self) -> & [& dyn Key] { & [& Self :: HELPER , & Self :: USERNAME , & Self :: USE_HTTP_PATH] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: URL_PARAMETER] } }
    };
}

impl_626!();