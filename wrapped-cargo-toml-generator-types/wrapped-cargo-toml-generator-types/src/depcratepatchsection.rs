// Generated macro for PatchSection (struct)
macro_rules! DepcratePatchSection {
() => {
// Module: crate
// Provides: {"PatchSection"}
// Dependencies: {}
# [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct PatchSection { # [serde (rename = "crates-io" , default , skip_serializing_if = "HashMap::is_empty")] pub crates_io : HashMap < String , Dependency > , }
};
}
