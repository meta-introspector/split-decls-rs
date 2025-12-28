macro_rules! deps {
    () => {
        MockCargoMetadataProvider!();
        DummyMetadata!();
        CargoMetadataProvider!();
        AnyMetadata!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl CargoMetadataProvider for MockCargoMetadataProvider { fn get_metadata (& self , _cargo_toml_path : & std :: path :: Path) -> Result < Box < dyn AnyMetadata > > { Ok (Box :: new (DummyMetadata :: default ())) } }
    };
}

impl_8!()