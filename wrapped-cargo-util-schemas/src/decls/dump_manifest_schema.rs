macro_rules! deps {
    () => {
        TomlManifest!();
    };
}

macro_rules! dump_manifest_schema {
    () => {
        deps!();
        # [cfg (feature = "unstable-schema")] # [test] fn dump_manifest_schema () { let schema = schemars :: schema_for ! (crate :: manifest :: TomlManifest) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../../manifest.schema.json") . raw ()) ; }
    };
}

dump_manifest_schema!()