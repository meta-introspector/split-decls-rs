// Generated macro for impl_1156 (impl)
macro_rules! Depcrate_core_config_toml_rustimpl_1156 {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"impl_1156"}
// Dependencies: {}
impl serde :: de :: Visitor < '_ > for OptimizeVisitor { type Value = RustOptimize ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { formatter . write_str (r#"one of: 0, 1, 2, 3, "s", "z", true, false"#) } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { if matches ! (value , "s" | "z") { Ok (RustOptimize :: String (value . to_string ())) } else { Err (serde :: de :: Error :: custom (format_optimize_error_msg (value))) } } fn visit_i64 < E > (self , value : i64) -> Result < Self :: Value , E > where E : serde :: de :: Error , { if matches ! (value , 0 ..= 3) { Ok (RustOptimize :: Int (value as u8)) } else { Err (serde :: de :: Error :: custom (format_optimize_error_msg (value))) } } fn visit_bool < E > (self , value : bool) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (RustOptimize :: Bool (value)) } }
};
}
