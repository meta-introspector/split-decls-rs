// Generated macro for deserialize_non_empty_vec (function)
macro_rules! Depcrate_requestdeserialize_non_empty_vec {
() => {
// Module: crate::request
// Provides: {"deserialize_non_empty_vec"}
// Dependencies: {}
fn deserialize_non_empty_vec < 'de , D , T > (deserializer : D) -> Result < Vec < T > , D :: Error > where D : Deserializer < 'de > , T : Deserialize < 'de > , { use serde :: de :: Error as _ ; let v = < Vec < T > > :: deserialize (deserializer) ? ; if v . is_empty () { Err (D :: Error :: invalid_length (0 , & "a non-empty sequence")) } else { Ok (v) } }
};
}
