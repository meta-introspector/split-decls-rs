macro_rules! deps {
    () => {
        TomlDebugInfo!();
        Result!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'de > de :: Deserialize < 'de > for TomlDebugInfo { fn deserialize < D > (d : D) -> Result < TomlDebugInfo , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; let expecting = "a boolean, 0, 1, 2, \"none\", \"limited\", \"full\", \"line-tables-only\", or \"line-directives-only\"" ; UntaggedEnumVisitor :: new () . expecting (expecting) . bool (| value | { Ok (if value { TomlDebugInfo :: Full } else { TomlDebugInfo :: None }) }) . i64 (| value | { let debuginfo = match value { 0 => TomlDebugInfo :: None , 1 => TomlDebugInfo :: Limited , 2 => TomlDebugInfo :: Full , _ => { return Err (serde_untagged :: de :: Error :: invalid_value (Unexpected :: Signed (value) , & expecting ,)) ; } } ; Ok (debuginfo) }) . string (| value | { let debuginfo = match value { "none" => TomlDebugInfo :: None , "limited" => TomlDebugInfo :: Limited , "full" => TomlDebugInfo :: Full , "line-directives-only" => TomlDebugInfo :: LineDirectivesOnly , "line-tables-only" => TomlDebugInfo :: LineTablesOnly , _ => { return Err (serde_untagged :: de :: Error :: invalid_value (Unexpected :: Str (value) , & expecting ,)) ; } } ; Ok (debuginfo) }) . deserialize (d) } }
    };
}

impl_134!();