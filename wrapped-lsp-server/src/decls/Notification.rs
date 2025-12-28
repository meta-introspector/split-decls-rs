macro_rules! Notification {
    () => {
        # [derive (Debug , Serialize , Deserialize , Clone)] pub struct Notification { pub method : String , # [serde (default = "serde_json::Value::default")] # [serde (skip_serializing_if = "serde_json::Value::is_null")] pub params : serde_json :: Value , }
    };
}

Notification!();