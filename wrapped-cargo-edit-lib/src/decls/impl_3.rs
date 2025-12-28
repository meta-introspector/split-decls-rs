macro_rules! deps {
    () => {
        AnyMetadata!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (feature = "real_cargo_metadata")] impl AnyMetadata for cargo_metadata :: Metadata { }
    };
}

impl_3!()