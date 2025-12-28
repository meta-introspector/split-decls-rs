macro_rules! deps {
    () => {
        DummyCargoTomlParser!();
    };
}

macro_rules! CurrentCargoTomlParser {
    () => {
        deps!();
        # [cfg (not (feature = "toml_edit_enabled"))] pub type CurrentCargoTomlParser = DummyCargoTomlParser ;
    };
}

CurrentCargoTomlParser!();