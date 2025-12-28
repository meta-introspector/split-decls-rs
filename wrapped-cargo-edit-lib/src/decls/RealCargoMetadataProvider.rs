macro_rules! RealCargoMetadataProvider {
    () => {
        # [cfg (feature = "real_cargo_metadata")] pub struct RealCargoMetadataProvider ;
    };
}

RealCargoMetadataProvider!()