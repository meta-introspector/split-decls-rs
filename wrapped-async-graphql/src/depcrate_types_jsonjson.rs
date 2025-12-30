// Generated macro for Json (struct)
macro_rules! Depcrate_types_jsonJson {
() => {
// Module: crate::types::json
// Provides: {"Json"}
// Dependencies: {}
# [doc = " A scalar that can represent any JSON value."] # [doc = ""] # [doc = " If the inner type cannot be serialized as JSON (e.g. it has non-string keys)"] # [doc = " it will be `null`."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq , Hash , Default)] # [serde (transparent)] pub struct Json < T > (pub T) ;
};
}
