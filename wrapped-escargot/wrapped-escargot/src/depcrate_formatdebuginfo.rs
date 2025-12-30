// Generated macro for DebugInfo (enum)
macro_rules! Depcrate_formatDebugInfo {
() => {
// Module: crate::format
// Provides: {"DebugInfo"}
// Dependencies: {}
# [doc = " The amount of debug info. 0 for none, 1 for limited, 2 for full"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [serde (untagged)] # [non_exhaustive] pub enum DebugInfo < 'a > { # [doc = " 0 for none, 1 for limited, 2 for full"] Level (u32) , # [doc = " none, limited, full, etc"] # [serde (borrow)] Name (CowStr < 'a >) , }
};
}
