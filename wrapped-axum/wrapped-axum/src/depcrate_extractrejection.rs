// Generated macro for rejection (module)
macro_rules! Depcrate_extractrejection {
() => {
// Module: crate::extract
// Provides: {"rejection"}
// Dependencies: {}
# [doc = " Rejection response types."] pub mod rejection { use async_graphql :: ParseRequestError ; use axum :: { body :: Body , http , http :: StatusCode , response :: { IntoResponse , Response } , } ; # [doc = " Rejection used for [`GraphQLRequest`](GraphQLRequest)."] pub struct GraphQLRejection (pub ParseRequestError) ; impl IntoResponse for GraphQLRejection { fn into_response (self) -> Response { match self . 0 { ParseRequestError :: PayloadTooLarge => http :: Response :: builder () . status (StatusCode :: PAYLOAD_TOO_LARGE) . body (Body :: empty ()) . unwrap () , bad_request => http :: Response :: builder () . status (StatusCode :: BAD_REQUEST) . body (Body :: from (format ! ("{:?}" , bad_request))) . unwrap () , } } } impl From < ParseRequestError > for GraphQLRejection { fn from (err : ParseRequestError) -> Self { GraphQLRejection (err) } } }
};
}
