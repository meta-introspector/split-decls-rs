// Generated macro for SystemBlock (struct)
macro_rules! Depcrate_modelsSystemBlock {
() => {
// Module: crate::models
// Provides: {"SystemBlock"}
// Dependencies: {}
# [doc = " System message block"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct SystemBlock { pub r#type : String , pub text : String , # [serde (skip_serializing_if = "Option::is_none")] pub cache_control : Option < serde_json :: Value > , }
};
}
