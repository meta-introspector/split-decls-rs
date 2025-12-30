// Generated macro for impl_12 (impl)
macro_rules! Depcrate_extractimpl_12 {
() => {
// Module: crate::extract
// Provides: {"impl_12"}
// Dependencies: {}
impl < S , R > FromRequest < S > for GraphQLBatchRequest < R > where S : Send + Sync , R : IntoResponse + From < ParseRequestError > , { type Rejection = R ; async fn from_request (req : Request , _state : & S) -> Result < Self , Self :: Rejection > { if req . method () == Method :: GET { let uri = req . uri () ; let res = async_graphql :: http :: parse_query_string (uri . query () . unwrap_or_default ()) . map_err (| err | { ParseRequestError :: Io (std :: io :: Error :: other (format ! ("failed to parse graphql request from uri query: {}" , err))) }) ; Ok (Self (async_graphql :: BatchRequest :: Single (res ?) , PhantomData)) } else { let content_type = req . headers () . get (http :: header :: CONTENT_TYPE) . and_then (| value | value . to_str () . ok ()) . map (ToString :: to_string) ; let body_stream = req . into_body () . into_data_stream () . map_err (| err | std :: io :: Error :: other (err . to_string ())) ; let body_reader = tokio_util :: io :: StreamReader :: new (body_stream) . compat () ; Ok (Self (async_graphql :: http :: receive_batch_body (content_type , body_reader , MultipartOptions :: default () ,) . await ? , PhantomData ,)) } } }
};
}
