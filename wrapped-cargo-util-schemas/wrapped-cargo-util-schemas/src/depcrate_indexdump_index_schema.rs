// Generated macro for dump_index_schema (function)
macro_rules! Depcrate_indexdump_index_schema {
() => {
// Module: crate::index
// Provides: {"dump_index_schema"}
// Dependencies: {}
# [cfg (feature = "unstable-schema")] # [test] fn dump_index_schema () { let schema = schemars :: schema_for ! (crate :: index :: IndexPackage <'_ >) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../index.schema.json") . raw ()) ; }
};
}
