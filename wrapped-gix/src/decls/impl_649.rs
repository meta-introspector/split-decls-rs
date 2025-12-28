macro_rules! deps {
    () => {
        Section!();
        Key!();
        Fetch!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl Section for Fetch { fn name (& self) -> & str { "fetch" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NEGOTIATION_ALGORITHM , # [cfg (feature = "attributes")] & Self :: RECURSE_SUBMODULES ,] } }
    };
}

impl_649!();