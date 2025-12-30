// Generated macro for deserialize_non_empty_batch (function)
macro_rules! Depcrate_httpdeserialize_non_empty_batch {
() => {
// Module: crate::http
// Provides: {"deserialize_non_empty_batch"}
// Dependencies: {}
fn deserialize_non_empty_batch < 'de , D , T > (deserializer : D) -> Result < Vec < T > , D :: Error > where D : de :: Deserializer < 'de > , T : Deserialize < 'de > , { use de :: Error as _ ; let v = Vec :: < T > :: deserialize (deserializer) ? ; if v . is_empty () { Err (D :: Error :: invalid_length (0 , & "non-empty batch of GraphQL requests" ,)) } else { Ok (v) } }
};
}
