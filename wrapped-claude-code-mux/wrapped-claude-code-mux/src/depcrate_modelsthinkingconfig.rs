// Generated macro for ThinkingConfig (struct)
macro_rules! Depcrate_modelsThinkingConfig {
() => {
// Module: crate::models
// Provides: {"ThinkingConfig"}
// Dependencies: {}
# [doc = " Thinking/reasoning configuration for Plan Mode"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct ThinkingConfig { pub r#type : String , # [serde (skip_serializing_if = "Option::is_none")] pub budget_tokens : Option < u32 > , }
};
}
