// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl GraphQLQuery { # [doc = " Shortcut method to execute the request on the schema."] pub async fn execute < E > (self , executor : & E) -> GraphQLResponse where E : Executor , { let request : GraphQLRequest = self . into () ; request . execute (executor) . await } }
};
}
