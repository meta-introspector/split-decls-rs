// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl GraphQLRequest { # [doc = " Shortcut method to execute the request on the schema."] pub async fn execute < E > (self , executor : & E) -> GraphQLResponse where E : Executor , { GraphQLResponse (executor . execute (self . 0) . await . into ()) } # [doc = " Insert some data for this request."] # [must_use] pub fn data < D : Any + Send + Sync > (mut self , data : D) -> Self { self . 0 . data . insert (data) ; self } }
};
}
