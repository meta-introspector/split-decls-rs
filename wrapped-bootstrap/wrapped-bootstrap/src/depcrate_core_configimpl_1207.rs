// Generated macro for impl_1207 (impl)
macro_rules! Depcrate_core_configimpl_1207 {
() => {
// Module: crate::core::config
// Provides: {"impl_1207"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for DebuginfoLevel { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { use serde :: de :: Error ; Ok (match Deserialize :: deserialize (deserializer) ? { StringOrInt :: String (s) if s == "none" => DebuginfoLevel :: None , StringOrInt :: Int (0) => DebuginfoLevel :: None , StringOrInt :: String (s) if s == "line-directives-only" => { DebuginfoLevel :: LineDirectivesOnly } StringOrInt :: String (s) if s == "line-tables-only" => DebuginfoLevel :: LineTablesOnly , StringOrInt :: String (s) if s == "limited" => DebuginfoLevel :: Limited , StringOrInt :: Int (1) => DebuginfoLevel :: Limited , StringOrInt :: String (s) if s == "full" => DebuginfoLevel :: Full , StringOrInt :: Int (2) => DebuginfoLevel :: Full , StringOrInt :: Int (n) => { let other = serde :: de :: Unexpected :: Signed (n) ; return Err (D :: Error :: invalid_value (other , & "expected 0, 1, or 2")) ; } StringOrInt :: String (s) => { let other = serde :: de :: Unexpected :: Str (& s) ; return Err (D :: Error :: invalid_value (other , & "expected none, line-tables-only, limited, or full" ,)) ; } }) } }
};
}
