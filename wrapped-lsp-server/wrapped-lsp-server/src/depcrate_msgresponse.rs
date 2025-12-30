// Generated macro for Response (struct)
macro_rules! Depcrate_msgResponse {
() => {
// Module: crate::msg
// Provides: {"Response"}
// Dependencies: {}
# [derive (Debug , Serialize , Deserialize , Clone)] pub struct Response { pub id : RequestId , # [serde (skip_serializing_if = "Option::is_none" , default)] pub result : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none" , default)] pub error : Option < ResponseError > , }
};
}
