// Generated macro for serde (module)
macro_rules! Depcrate_naiveserde {
() => {
// Module: crate::naive
// Provides: {"serde"}
// Dependencies: {}
# [doc = " Serialization/Deserialization of `NaiveDateTime` in alternate formats"] # [doc = ""] # [doc = " The various modules in here are intended to be used with serde's [`with` annotation] to"] # [doc = " serialize as something other than the default ISO 8601 format."] # [doc = ""] # [doc = " [`with` annotation]: https://serde.rs/field-attrs.html#with"] # [cfg (feature = "serde")] pub mod serde { pub use super :: datetime :: serde :: * ; }
};
}
