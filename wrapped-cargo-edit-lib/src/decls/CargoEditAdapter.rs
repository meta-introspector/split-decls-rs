macro_rules! deps {
    () => {
        CargoMetadataProvider!();
    };
}

macro_rules! CargoEditAdapter {
    () => {
        deps!();
        pub trait CargoEditAdapter : Send + Sync { fn generate_cargo_config (& self , git_adapter : & dyn GitAdapter , cargo_metadata_provider : & dyn CargoMetadataProvider ,) -> Result < String , anyhow :: Error > ; }
    };
}

CargoEditAdapter!();