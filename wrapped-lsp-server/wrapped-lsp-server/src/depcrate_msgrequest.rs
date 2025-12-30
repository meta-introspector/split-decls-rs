// Generated macro for Request (struct)
macro_rules! Depcrate_msgRequest {
() => {
// Module: crate::msg
// Provides: {"Request"}
// Dependencies: {}
# [derive (Debug , Serialize , Deserialize , Clone)] pub struct Request { pub id : RequestId , pub method : String , # [serde (default = "serde_json::Value::default")] # [serde (skip_serializing_if = "serde_json::Value::is_null")] pub params : serde_json :: Value , }
};
}
