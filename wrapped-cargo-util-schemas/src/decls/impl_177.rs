macro_rules! deps {
    () => {
        Result!();
        InvalidCargoFeatures!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for InvalidCargoFeatures { fn deserialize < D > (_d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; Err (D :: Error :: custom ("the field `cargo-features` should be set at the top of Cargo.toml before any tables" ,)) } }
    };
}

impl_177!();