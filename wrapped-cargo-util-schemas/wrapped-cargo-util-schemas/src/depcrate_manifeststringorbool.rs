// Generated macro for StringOrBool (enum)
macro_rules! Depcrate_manifestStringOrBool {
() => {
// Module: crate::manifest
// Provides: {"StringOrBool"}
// Dependencies: {}
# [derive (Clone , Debug , Serialize , Eq , PartialEq)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum StringOrBool { String (String) , Bool (bool) , }
};
}
