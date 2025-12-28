macro_rules! deps {
    () => {
        Key!();
        Section!();
        Protocol!();
    };
}

macro_rules! impl_709 {
    () => {
        deps!();
        impl Section for Protocol { fn name (& self) -> & str { "protocol" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ALLOW , & Self :: VERSION] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: NAME_PARAMETER] } }
    };
}

impl_709!()