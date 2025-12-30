// Generated macro for dump_manifest_schema (function)
macro_rules! Depcrate_manifestdump_manifest_schema {
() => {
// Module: crate::manifest
// Provides: {"dump_manifest_schema"}
// Dependencies: {}
# [cfg (feature = "unstable-schema")] # [test] fn dump_manifest_schema () { let schema = schemars :: schema_for ! (crate :: manifest :: TomlManifest) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../../manifest.schema.json") . raw ()) ; }
};
}
