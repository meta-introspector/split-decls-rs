macro_rules! deps {
    () => {
        TomlPackageBuild!();
        Result!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for TomlPackageBuild { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { UntaggedEnumVisitor :: new () . bool (| b | Ok (TomlPackageBuild :: Auto (b))) . string (| s | Ok (TomlPackageBuild :: SingleScript (s . to_owned ()))) . seq (| value | value . deserialize () . map (TomlPackageBuild :: MultipleScript)) . deserialize (deserializer) } }
    };
}

impl_184!();