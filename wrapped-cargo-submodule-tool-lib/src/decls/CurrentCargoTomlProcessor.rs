macro_rules! deps {
    () => {
        DummyCargoTomlProcessor!();
    };
}

macro_rules! CurrentCargoTomlProcessor {
    () => {
        deps!();
        # [cfg (not (feature = "toml_edit_enabled"))] pub type CurrentCargoTomlProcessor = DummyCargoTomlProcessor ;
    };
}

CurrentCargoTomlProcessor!();