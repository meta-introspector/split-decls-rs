macro_rules! deps {
    () => {
        RealCargoMetadataProvider!();
        CargoMetadataProvider!();
        AnyMetadata!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (feature = "real_cargo_metadata")] impl CargoMetadataProvider for RealCargoMetadataProvider { fn get_metadata (& self , cargo_toml_path : & std :: path :: Path) -> Result < Box < dyn AnyMetadata > > { let metadata = cargo_metadata :: MetadataCommand :: new () . manifest_path (cargo_toml_path) . exec () . map_err (| e | anyhow :: anyhow ! ("Failed to get cargo metadata: {}" , e)) ? ; Ok (Box :: new (metadata)) } }
    };
}

impl_6!();