macro_rules! deps {
    () => {
        TomlOptLevel!();
        Result!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for TomlOptLevel { fn deserialize < D > (d : D) -> Result < TomlOptLevel , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; UntaggedEnumVisitor :: new () . expecting ("an optimization level") . i64 (| value | Ok (TomlOptLevel (value . to_string ()))) . string (| value | { if value == "s" || value == "z" { Ok (TomlOptLevel (value . to_string ())) } else { Err (serde_untagged :: de :: Error :: custom (format ! ("must be `0`, `1`, `2`, `3`, `s` or `z`, \
                         but found the string: \"{}\"" , value))) } }) . deserialize (d) } }
    };
}

impl_130!()