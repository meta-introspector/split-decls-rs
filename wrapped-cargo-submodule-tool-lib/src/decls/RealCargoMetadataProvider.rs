macro_rules! RealCargoMetadataProvider {
    () => {
        # [cfg (not (feature = "nix_generation"))] pub struct RealCargoMetadataProvider ;
    };
}

RealCargoMetadataProvider!()