// Generated macro for impl_349 (impl)
macro_rules! Depcrate_configimpl_349 {
() => {
// Module: crate::config
// Provides: {"impl_349"}
// Dependencies: {}
impl FullConfigInput { fn from_json (mut json : serde_json :: Value , error_sink : & mut Vec < (String , serde_json :: Error) > ,) -> FullConfigInput { FullConfigInput { global : GlobalConfigInput :: from_json (& mut json , error_sink) , local : LocalConfigInput :: from_json (& mut json , error_sink) , client : ClientConfigInput :: from_json (& mut json , error_sink) , workspace : WorkspaceConfigInput :: from_json (& mut json , error_sink) , } } fn schema_fields () -> Vec < SchemaField > { let mut fields = Vec :: new () ; GlobalConfigInput :: schema_fields (& mut fields) ; LocalConfigInput :: schema_fields (& mut fields) ; ClientConfigInput :: schema_fields (& mut fields) ; WorkspaceConfigInput :: schema_fields (& mut fields) ; fields . sort_by_key (| & (x , ..) | x) ; fields . iter () . tuple_windows () . for_each (| (a , b) | assert ! (a . 0 != b . 0 , "{a:?} duplicate field")) ; fields } fn json_schema () -> serde_json :: Value { schema (& Self :: schema_fields ()) } # [cfg (test)] fn manual () -> String { manual (& Self :: schema_fields ()) } }
};
}
