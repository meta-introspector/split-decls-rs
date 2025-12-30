// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
# [rocket :: async_trait] impl < 'r > FromData < 'r > for GraphQLBatchRequest { type Error = ParseRequestError ; async fn from_data (req : & 'r rocket :: Request < '_ > , data : Data < 'r >) -> data :: Outcome < 'r , Self > { let opts : MultipartOptions = req . rocket () . state () . copied () . unwrap_or_default () ; let request = async_graphql :: http :: receive_batch_body (req . headers () . get_one ("Content-Type") , data . open (req . limits () . get ("graphql") . unwrap_or_else (| | 128 . kibibytes ()) ,) . compat () , opts ,) . await ; match request { Ok (request) => data :: Outcome :: Success (Self (request)) , Err (e) => data :: Outcome :: Error ((match e { ParseRequestError :: PayloadTooLarge => Status :: PayloadTooLarge , _ => Status :: BadRequest , } , e ,)) , } } }
};
}
