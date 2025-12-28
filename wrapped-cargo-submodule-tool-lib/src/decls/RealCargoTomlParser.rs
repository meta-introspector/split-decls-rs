macro_rules! RealCargoTomlParser {
    () => {
        # [cfg (feature = "toml_edit_enabled")] pub struct RealCargoTomlParser ;
    };
}

RealCargoTomlParser!()