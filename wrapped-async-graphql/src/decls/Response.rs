macro_rules! deps {
    () => {
        Data!();
        CacheControl!();
        Query!();
        ServerError!();
        Extensions!();
    };
}

macro_rules! Response {
    () => {
        deps!();
        # [doc = " Query response"] # [non_exhaustive] # [derive (Debug , Default , Serialize , Deserialize , PartialEq)] pub struct Response { # [doc = " Data of query result"] # [serde (default)] pub data : Value , # [doc = " Extensions result"] # [serde (skip_serializing_if = "BTreeMap::is_empty" , default)] pub extensions : BTreeMap < String , Value > , # [doc = " Cache control value"] # [serde (skip)] pub cache_control : CacheControl , # [doc = " Errors"] # [serde (skip_serializing_if = "Vec::is_empty" , default)] pub errors : Vec < ServerError > , # [doc = " HTTP headers"] # [serde (skip)] pub http_headers : http :: HeaderMap , }
    };
}

Response!()