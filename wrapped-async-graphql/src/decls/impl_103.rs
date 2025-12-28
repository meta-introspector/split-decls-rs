macro_rules! deps {
    () => {
        BatchRequest!();
        Result!();
        Request!();
        ParseRequestError!();
        Any!();
        IntrospectionMode!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl BatchRequest { # [doc = " Attempt to convert the batch request into a single request."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if the batch request is a list of requests with a message saying"] # [doc = " that batch requests aren't supported."] pub fn into_single (self) -> Result < Request , ParseRequestError > { match self { Self :: Single (req) => Ok (req) , Self :: Batch (_) => Err (ParseRequestError :: UnsupportedBatch) , } } # [doc = " Returns an iterator over the requests."] pub fn iter (& self) -> impl Iterator < Item = & Request > { match self { BatchRequest :: Single (request) => { Box :: new (std :: iter :: once (request)) as Box < dyn Iterator < Item = & Request > > } BatchRequest :: Batch (requests) => Box :: new (requests . iter ()) , } } # [doc = " Returns an iterator that allows modifying each request."] pub fn iter_mut (& mut self) -> impl Iterator < Item = & mut Request > { match self { BatchRequest :: Single (request) => { Box :: new (std :: iter :: once (request)) as Box < dyn Iterator < Item = & mut Request > > } BatchRequest :: Batch (requests) => Box :: new (requests . iter_mut ()) , } } # [doc = " Specify the variables for each requests."] # [must_use] pub fn variables (mut self , variables : Variables) -> Self { for request in self . iter_mut () { request . variables = variables . clone () ; } self } # [doc = " Insert some data for  for each requests."] # [must_use] pub fn data < D : Any + Clone + Send + Sync > (mut self , data : D) -> Self { for request in self . iter_mut () { request . data . insert (data . clone ()) ; } self } # [doc = " Disable introspection queries for each request."] # [must_use] pub fn disable_introspection (mut self) -> Self { for request in self . iter_mut () { request . introspection_mode = IntrospectionMode :: Disabled ; } self } # [doc = " Only allow introspection queries for each request."] # [must_use] pub fn introspection_only (mut self) -> Self { for request in self . iter_mut () { request . introspection_mode = IntrospectionMode :: IntrospectionOnly ; } self } }
    };
}

impl_103!()