// Generated macro for impl_491 (impl)
macro_rules! Depcrate_contextimpl_491 {
() => {
// Module: crate::context
// Provides: {"impl_491"}
// Dependencies: {}
impl QueryEnv { # [doc (hidden)] pub fn new (inner : QueryEnvInner) -> QueryEnv { QueryEnv (Arc :: new (inner)) } # [doc (hidden)] pub fn create_context < 'a , T > (& 'a self , schema_env : & 'a SchemaEnv , path_node : Option < QueryPathNode < 'a > > , item : T , execute_data : Option < & 'a Data > ,) -> ContextBase < 'a , T > { ContextBase { path_node , is_for_introspection : false , item , schema_env , query_env : self , execute_data , } } }
};
}
