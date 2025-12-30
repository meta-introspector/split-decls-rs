// Generated macro for tests (module)
macro_rules! Depcrate_util_ident_string_serdetests {
() => {
// Module: crate::util::ident_string::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn roundtrip () { let raw_ident = r#""ident""# ; let deserialized : IdentString = serde_json :: from_str (& raw_ident) . expect ("ident must be valid") ; let serialized = serde_json :: to_string (& deserialized) . expect ("ident must be serializable") ; assert_eq ! (raw_ident , serialized) ; } }
};
}
