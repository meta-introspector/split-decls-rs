macro_rules! deps {
    () => {
        DummyMetadata!();
        AnyMetadata!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        # [cfg (not (feature = "real_cargo_metadata"))] impl AnyMetadata for DummyMetadata { }
    };
}

impl_2!()