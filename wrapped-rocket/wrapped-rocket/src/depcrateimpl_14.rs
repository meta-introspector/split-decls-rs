// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
# [rocket :: async_trait] impl < 'r > FromData < 'r > for GraphQLRequest { type Error = ParseRequestError ; async fn from_data (req : & 'r rocket :: Request < '_ > , data : Data < 'r >) -> data :: Outcome < 'r , Self > { GraphQLBatchRequest :: from_data (req , data) . await . and_then (| request | match request . 0 . into_single () { Ok (single) => data :: Outcome :: Success (Self (single)) , Err (e) => data :: Outcome :: Error ((Status :: BadRequest , e)) , }) } }
};
}
