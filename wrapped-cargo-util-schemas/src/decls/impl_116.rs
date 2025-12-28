macro_rules! deps {
    () => {
        Result!();
        TomlDependency!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < 'de , P : Deserialize < 'de > + Clone > de :: Deserialize < 'de > for TomlDependency < P > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; let expected = "a version string like \"0.9.8\" or a \
                     detailed dependency like { version = \"0.9.8\" }" ; UntaggedEnumVisitor :: new () . expecting (expected) . string (| value | Ok (TomlDependency :: Simple (value . to_owned ()))) . bool (| value | { let expected = format ! ("invalid type: boolean `{value}`, expected {expected}") ; let err = if value { format ! ("{expected}\n\
                    note: if you meant to use a workspace member, you can write\n \
                      dep.workspace = {value}") } else { expected } ; Err (serde_untagged :: de :: Error :: custom (err)) }) . map (| value | value . deserialize () . map (TomlDependency :: Detailed)) . deserialize (deserializer) } }
    };
}

impl_116!()