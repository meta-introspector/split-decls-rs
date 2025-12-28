macro_rules! deps {
    () => {
        Extensions!();
        IntrospectionMode!();
        Data!();
        UploadValue!();
    };
}

macro_rules! Request {
    () => {
        deps!();
        # [doc = " GraphQL request."] # [doc = ""] # [doc = " This can be deserialized from a structure of the query string, the operation"] # [doc = " name and the variables. The names are all in `camelCase` (e.g."] # [doc = " `operationName`)."] # [non_exhaustive] # [derive (Serialize , Deserialize)] # [serde (rename_all = "camelCase")] pub struct Request { # [doc = " The query source of the request."] # [serde (default)] pub query : String , # [doc = " The operation name of the request."] # [serde (default , rename = "operationName")] pub operation_name : Option < String > , # [doc = " The variables of the request."] # [serde (default)] pub variables : Variables , # [doc = " Uploads sent with the request."] # [serde (skip)] pub uploads : Vec < UploadValue > , # [doc = " The data of the request that can be accessed through `Context::data`."] # [doc = ""] # [doc = " **This data is only valid for this request**"] # [serde (skip)] pub data : Data , # [doc = " The extensions config of the request."] # [serde (default)] pub extensions : Extensions , # [serde (skip)] pub (crate) parsed_query : Option < ExecutableDocument > , # [doc = " Sets the introspection mode for this request (defaults to"] # [doc = " [IntrospectionMode::Enabled])."] # [serde (skip)] pub introspection_mode : IntrospectionMode , }
    };
}

Request!();