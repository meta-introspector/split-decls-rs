// Generated macro for Tool (struct)
macro_rules! Depcrate_modelsTool {
() => {
// Module: crate::models
// Provides: {"Tool"}
// Dependencies: {}
# [doc = " Tool definition for function calling"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct Tool { # [serde (skip_serializing_if = "Option::is_none")] pub r#type : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub name : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub description : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub input_schema : Option < serde_json :: Value > , }
};
}
