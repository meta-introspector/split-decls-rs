macro_rules! deps {
    () => {
        AnyMetadata!();
    };
}

macro_rules! CargoMetadataProvider {
    () => {
        deps!();
        pub trait CargoMetadataProvider : Send + Sync { fn get_metadata (& self , cargo_toml_path : & std :: path :: Path) -> Result < Box < dyn AnyMetadata > > ; }
    };
}

CargoMetadataProvider!();