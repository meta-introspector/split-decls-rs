// Generated macro for impl_302 (impl)
macro_rules! Depcrate_valueimpl_302 {
() => {
// Module: crate::value
// Provides: {"impl_302"}
// Dependencies: {}
impl Value { # [doc = " Tries to deserialize this [`Value`] into `T`."] pub fn into_rust < T > (self) -> Result < T > where T : DeserializeOwned , { T :: deserialize (self) } }
};
}
