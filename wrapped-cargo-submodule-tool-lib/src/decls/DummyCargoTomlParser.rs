macro_rules! DummyCargoTomlParser {
    () => {
        # [cfg (not (feature = "toml_edit_enabled"))] pub struct DummyCargoTomlParser ;
    };
}

DummyCargoTomlParser!()