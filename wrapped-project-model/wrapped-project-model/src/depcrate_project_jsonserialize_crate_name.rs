// Generated macro for serialize_crate_name (function)
macro_rules! Depcrate_project_jsonserialize_crate_name {
() => {
// Module: crate::project_json
// Provides: {"serialize_crate_name"}
// Dependencies: {}
fn serialize_crate_name < S > (name : & CrateName , se : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { se . serialize_str (name . as_str ()) }
};
}
