macro_rules! deps {
    () => {
        Request!();
        Data!();
        Result!();
        Any!();
        UploadValue!();
        ServerError!();
        IntrospectionMode!();
        Object!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl Request { # [doc = " Create a request object with query source."] pub fn new (query : impl Into < String >) -> Self { Self { query : query . into () , operation_name : None , variables : Variables :: default () , uploads : Vec :: default () , data : Data :: default () , extensions : Default :: default () , parsed_query : None , introspection_mode : IntrospectionMode :: Enabled , } } # [doc = " Specify the operation name of the request."] # [must_use] pub fn operation_name < T : Into < String > > (self , name : T) -> Self { Self { operation_name : Some (name . into ()) , .. self } } # [doc = " Specify the variables."] # [must_use] pub fn variables (self , variables : Variables) -> Self { Self { variables , .. self } } # [doc = " Insert some data for this request."] # [must_use] pub fn data < D : Any + Send + Sync > (mut self , data : D) -> Self { self . data . insert (data) ; self } # [doc = " Disable introspection queries for this request."] # [must_use] pub fn disable_introspection (mut self) -> Self { self . introspection_mode = IntrospectionMode :: Disabled ; self } # [doc = " Only allow introspection queries for this request."] # [must_use] pub fn only_introspection (mut self) -> Self { self . introspection_mode = IntrospectionMode :: IntrospectionOnly ; self } # [inline] # [doc = " Performs parsing of query ahead of execution."] # [doc = ""] # [doc = " This effectively allows to inspect query information, before passing"] # [doc = " request to schema for execution as long as query is valid."] pub fn parsed_query (& mut self) -> Result < & ExecutableDocument , ServerError > { if self . parsed_query . is_none () { match parse_query (& self . query) { Ok (parsed) => self . parsed_query = Some (parsed) , Err (error) => return Err (error . into ()) , } } Ok (self . parsed_query . as_ref () . unwrap ()) } # [doc = " Sets the parsed query into the request."] # [doc = ""] # [doc = " This is useful special with dynamic schema when the query has been"] # [doc = " parsed ahead of time. It can reduce performance overhead of parsing"] # [doc = " the query again."] pub fn set_parsed_query (& mut self , doc : ExecutableDocument) { self . parsed_query = Some (doc) ; } # [doc = " Set a variable to an upload value."] # [doc = ""] # [doc = " `var_path` is a dot-separated path to the item that begins with"] # [doc = " `variables`, for example `variables.files.2.content` is equivalent"] # [doc = " to the Rust code `request.variables[\"files\"][2][\"content\"]`. If no"] # [doc = " variable exists at the path this function won't do anything."] pub fn set_upload (& mut self , var_path : & str , upload : UploadValue) { fn variable_path < 'a > (variables : & 'a mut Variables , path : & str) -> Option < & 'a mut Value > { let mut parts = path . strip_prefix ("variables.") ? . split ('.') ; let initial = variables . get_mut (parts . next () . unwrap ()) ? ; parts . try_fold (initial , | current , part | match current { Value :: List (list) => part . parse :: < u32 > () . ok () . and_then (| idx | usize :: try_from (idx) . ok ()) . and_then (move | idx | list . get_mut (idx)) , Value :: Object (obj) => obj . get_mut (part) , _ => None , }) } let variable = match variable_path (& mut self . variables , var_path) { Some (variable) => variable , None => return , } ; self . uploads . push (upload) ; * variable = Value :: String (format ! ("#__graphql_file__:{}" , self . uploads . len () - 1)) ; } }
    };
}

impl_99!()