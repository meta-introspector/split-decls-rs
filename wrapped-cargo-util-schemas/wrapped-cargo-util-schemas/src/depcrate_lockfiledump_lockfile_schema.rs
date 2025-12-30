// Generated macro for dump_lockfile_schema (function)
macro_rules! Depcrate_lockfiledump_lockfile_schema {
() => {
// Module: crate::lockfile
// Provides: {"dump_lockfile_schema"}
// Dependencies: {}
# [cfg (feature = "unstable-schema")] # [test] fn dump_lockfile_schema () { let schema = schemars :: schema_for ! (crate :: lockfile :: TomlLockfile) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../lockfile.schema.json") . raw ()) ; }
};
}
