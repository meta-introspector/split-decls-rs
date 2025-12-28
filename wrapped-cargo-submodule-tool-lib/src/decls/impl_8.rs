macro_rules! deps {
    () => {
        DummyCargoMetadataProvider!();
        CargoMetadataProvider!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] impl CargoMetadataProvider for DummyCargoMetadataProvider { }
    };
}

impl_8!();