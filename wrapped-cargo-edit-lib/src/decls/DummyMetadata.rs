macro_rules! DummyMetadata {
    () => {
        # [cfg (not (feature = "real_cargo_metadata"))] # [derive (Debug , Clone , PartialEq , Eq , Default)] pub struct DummyMetadata ;
    };
}

DummyMetadata!()