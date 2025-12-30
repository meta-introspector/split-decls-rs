// Generated macro for Message (enum)
macro_rules! Depcrate_msgMessage {
() => {
// Module: crate::msg
// Provides: {"Message"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (untagged)] pub enum Message { Request (Request) , Response (Response) , Notification (Notification) , }
};
}
