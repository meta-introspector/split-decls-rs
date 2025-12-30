// Generated macro for impl_230 (impl)
macro_rules! Depcrate_manifestimpl_230 {
() => {
// Module: crate::manifest
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for InvalidCargoFeatures { fn deserialize < D > (_d : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { use serde :: de :: Error as _ ; Err (D :: Error :: custom ("the field `cargo-features` should be set at the top of Cargo.toml before any tables" ,)) } }
};
}
