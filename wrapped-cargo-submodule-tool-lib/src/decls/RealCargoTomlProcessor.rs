macro_rules! RealCargoTomlProcessor {
    () => {
        # [cfg (feature = "toml_edit_enabled")] pub struct RealCargoTomlProcessor ;
    };
}

RealCargoTomlProcessor!();