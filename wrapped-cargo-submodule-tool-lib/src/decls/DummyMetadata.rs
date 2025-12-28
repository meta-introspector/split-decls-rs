macro_rules! DummyMetadata {
    () => {
        # [cfg (not (feature = "nix_generation"))] # [derive (Debug)] pub struct DummyMetadata ;
    };
}

DummyMetadata!()