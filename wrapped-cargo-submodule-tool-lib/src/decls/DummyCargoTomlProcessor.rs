macro_rules! DummyCargoTomlProcessor {
    () => {
        # [cfg (not (feature = "toml_edit_enabled"))] pub struct DummyCargoTomlProcessor ;
    };
}

DummyCargoTomlProcessor!();