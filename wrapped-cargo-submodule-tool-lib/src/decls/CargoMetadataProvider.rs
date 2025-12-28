macro_rules! deps {
    () => {
        DummyMetadata!();
    };
}

macro_rules! CargoMetadataProvider {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] pub trait CargoMetadataProvider : Send + Sync { fn get_metadata (& self , cargo_toml_path : & Path) -> Result < DummyMetadata > { anyhow :: bail ! ("Dummy CargoMetadataProvider: get_metadata not implemented for {:?}" , cargo_toml_path) } }
    };
}

CargoMetadataProvider!();