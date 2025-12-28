macro_rules! deps {
    () => {
        ResponseError!();
        RequestId!();
    };
}

macro_rules! Response {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize , Clone)] pub struct Response { pub id : RequestId , # [serde (skip_serializing_if = "Option::is_none" , default)] pub result : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none" , default)] pub error : Option < ResponseError > , }
    };
}

Response!();