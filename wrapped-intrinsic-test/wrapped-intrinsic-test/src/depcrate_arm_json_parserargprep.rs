// Generated macro for ArgPrep (enum)
macro_rules! Depcrate_arm_json_parserArgPrep {
() => {
// Module: crate::arm::json_parser
// Provides: {"ArgPrep"}
// Dependencies: {}
# [derive (Deserialize , Debug)] # [serde (untagged , deny_unknown_fields)] pub enum ArgPrep { Register { # [serde (rename = "register")] # [allow (dead_code)] reg : String , } , Immediate { # [serde (rename = "minimum")] min : i64 , # [serde (rename = "maximum")] max : i64 , } , Nothing { } , }
};
}
