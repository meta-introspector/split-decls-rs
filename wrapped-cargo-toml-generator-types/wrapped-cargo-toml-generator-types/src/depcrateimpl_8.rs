// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Dependency { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; let value = toml :: Value :: deserialize (deserializer) ? ; match value { toml :: Value :: String (s) => Ok (Dependency :: Version (s)) , toml :: Value :: Table (_) => { let table = DependencyTable :: deserialize (value) . map_err (D :: Error :: custom) ? ; Ok (Dependency :: Table (table)) } _ => Err (D :: Error :: custom ("Invalid dependency format")) , } } }
};
}
