// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [rocket :: async_trait] impl < 'r , S > FromData < 'r > for GraphQLRequest < S > where S : ScalarValue , { type Error = String ; async fn from_data (req : & 'r Request < '_ > , data : Data < 'r > ,) -> data :: Outcome < 'r , Self , Self :: Error > { use rocket :: tokio :: io :: AsyncReadExt as _ ; let content_type = req . content_type () . map (| ct | (ct . top () . as_str () , ct . sub () . as_str ())) ; let is_json = match content_type { Some (("application" , "json")) => true , Some (("application" , "graphql")) => false , _ => return Outcome :: Forward ((data , Status :: UnsupportedMediaType)) , } ; Box :: pin (async move { let limit = req . limits () . get ("graphql") . unwrap_or_else (| | BODY_LIMIT . bytes ()) ; let mut reader = data . open (limit) ; let mut body = String :: new () ; if let Err (e) = reader . read_to_string (& mut body) . await { return Outcome :: Error ((Status :: InternalServerError , format ! ("{e:?}"))) ; } Outcome :: Success (GraphQLRequest (if is_json { match serde_json :: from_str (& body) { Ok (req) => req , Err (e) => return Outcome :: Error ((Status :: BadRequest , e . to_string ())) , } } else { GraphQLBatchRequest :: Single (http :: GraphQLRequest :: new (body , None , None)) })) }) . await } }
};
}
