macro_rules! DummyCargoMetadataProvider {
    () => {
        # [cfg (not (feature = "nix_generation"))] pub struct DummyCargoMetadataProvider ;
    };
}

DummyCargoMetadataProvider!();