macro_rules! deps {
    () => {
        Request!();
        BatchResponse!();
        BatchRequest!();
        Response!();
        Data!();
    };
}

macro_rules! Executor {
    () => {
        deps!();
        # [doc = " Represents a GraphQL executor"] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait Executor : Unpin + Clone + Send + Sync + 'static { # [doc = " Execute a GraphQL query."] # [cfg (feature = "boxed-trait")] async fn execute (& self , request : Request) -> Response ; # [doc = " Execute a GraphQL query."] # [cfg (not (feature = "boxed-trait"))] fn execute (& self , request : Request) -> impl Future < Output = Response > + Send ; # [doc = " Execute a GraphQL batch query."] # [cfg (feature = "boxed-trait")] async fn execute_batch (& self , batch_request : BatchRequest) -> BatchResponse { match batch_request { BatchRequest :: Single (request) => BatchResponse :: Single (self . execute (request) . await) , BatchRequest :: Batch (requests) => BatchResponse :: Batch (FuturesOrdered :: from_iter (requests . into_iter () . map (| request | self . execute (request)) ,) . collect () . await ,) , } } # [doc = " Execute a GraphQL batch query."] # [cfg (not (feature = "boxed-trait"))] fn execute_batch (& self , batch_request : BatchRequest ,) -> impl Future < Output = BatchResponse > + Send { async { match batch_request { BatchRequest :: Single (request) => BatchResponse :: Single (self . execute (request) . await) , BatchRequest :: Batch (requests) => BatchResponse :: Batch (FuturesOrdered :: from_iter (requests . into_iter () . map (| request | self . execute (request)) ,) . collect () . await ,) , } } } # [doc = " Execute a GraphQL subscription with session data."] fn execute_stream (& self , request : Request , session_data : Option < Arc < Data > > ,) -> BoxStream < 'static , Response > ; }
    };
}

Executor!()