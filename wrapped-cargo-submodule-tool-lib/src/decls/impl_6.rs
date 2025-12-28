macro_rules! deps {
    () => {
        CargoMetadataProvider!();
        RealCargoMetadataProvider!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] impl CargoMetadataProvider for RealCargoMetadataProvider { }
    };
}

impl_6!()