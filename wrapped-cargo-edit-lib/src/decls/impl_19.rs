macro_rules! deps {
    () => {
        CargoEditAdapter!();
        CargoEditAdapterImpl!();
        CargoMetadataProvider!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl CargoEditAdapter for CargoEditAdapterImpl { fn generate_cargo_config (& self , git_adapter : & dyn GitAdapter , cargo_metadata_provider : & dyn CargoMetadataProvider ,) -> Result < String , anyhow :: Error > { Ok ("[cargo]\nbuild-std = [\"core\", \"alloc\"]\n" . to_string ()) } }
    };
}

impl_19!()