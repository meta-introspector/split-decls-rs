macro_rules! deps {
    () => {
        QueryPathNode!();
        QueryEnv!();
        Query!();
        Data!();
        SchemaEnv!();
    };
}

macro_rules! ContextBase {
    () => {
        deps!();
        # [doc = " Query context."] # [doc = ""] # [doc = " **This type is not stable and should not be used directly.**"] # [derive (Clone)] pub struct ContextBase < 'a , T > { # [doc = " The current path node being resolved."] pub path_node : Option < QueryPathNode < 'a > > , # [doc = " If `true` means the current field is for introspection."] pub (crate) is_for_introspection : bool , # [doc (hidden)] pub item : T , # [doc (hidden)] pub schema_env : & 'a SchemaEnv , # [doc (hidden)] pub query_env : & 'a QueryEnv , # [doc (hidden)] pub execute_data : Option < & 'a Data > , }
    };
}

ContextBase!()