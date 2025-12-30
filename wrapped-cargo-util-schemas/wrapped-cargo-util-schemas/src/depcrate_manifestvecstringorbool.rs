// Generated macro for VecStringOrBool (enum)
macro_rules! Depcrate_manifestVecStringOrBool {
() => {
// Module: crate::manifest
// Provides: {"VecStringOrBool"}
// Dependencies: {}
# [derive (PartialEq , Clone , Debug , Serialize)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum VecStringOrBool { VecString (Vec < String >) , Bool (bool) , }
};
}
