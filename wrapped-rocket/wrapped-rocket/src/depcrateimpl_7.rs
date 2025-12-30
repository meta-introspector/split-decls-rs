// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl GraphQLBatchRequest { # [doc = " Shortcut method to execute the request on the executor."] pub async fn execute < E > (self , executor : & E) -> GraphQLResponse where E : Executor , { GraphQLResponse (executor . execute_batch (self . 0) . await) } }
};
}
