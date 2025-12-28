macro_rules! ResponseError {
    () => {
        # [derive (Debug , Serialize , Deserialize , Clone)] pub struct ResponseError { pub code : i32 , pub message : String , # [serde (skip_serializing_if = "Option::is_none" , default)] pub data : Option < serde_json :: Value > , }
    };
}

ResponseError!()