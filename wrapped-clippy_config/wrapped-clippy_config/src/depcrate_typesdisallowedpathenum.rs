// Generated macro for DisallowedPathEnum (enum)
macro_rules! Depcrate_typesDisallowedPathEnum {
() => {
// Module: crate::types
// Provides: {"DisallowedPathEnum"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize)] # [serde (untagged , deny_unknown_fields)] enum DisallowedPathEnum { Simple (String) , WithReason { path : String , reason : Option < String > , replacement : Option < String > , # [serde (rename = "allow-invalid")] allow_invalid : Option < bool > , } , }
};
}
