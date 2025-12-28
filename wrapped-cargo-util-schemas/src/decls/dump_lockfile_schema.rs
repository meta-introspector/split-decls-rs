macro_rules! deps {
    () => {
        TomlLockfile!();
    };
}

macro_rules! dump_lockfile_schema {
    () => {
        deps!();
        # [cfg (feature = "unstable-schema")] # [test] fn dump_lockfile_schema () { let schema = schemars :: schema_for ! (crate :: lockfile :: TomlLockfile) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../lockfile.schema.json") . raw ()) ; }
    };
}

dump_lockfile_schema!()