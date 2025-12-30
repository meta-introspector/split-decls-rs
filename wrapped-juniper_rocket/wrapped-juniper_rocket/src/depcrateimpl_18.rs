// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'r , 'o : 'r > Responder < 'r , 'o > for GraphQLResponse { fn respond_to (self , _req : & 'r Request < '_ >) -> response :: Result < 'o > { let GraphQLResponse (status , body) = self ; Response :: build () . header (ContentType :: new ("application" , "json")) . status (status) . sized_body (body . len () , Cursor :: new (body)) . ok () } }
};
}
