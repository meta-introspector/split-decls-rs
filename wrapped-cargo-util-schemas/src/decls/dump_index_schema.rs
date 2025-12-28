macro_rules! deps {
    () => {
        IndexPackage!();
    };
}

macro_rules! dump_index_schema {
    () => {
        deps!();
        # [cfg (feature = "unstable-schema")] # [test] fn dump_index_schema () { let schema = schemars :: schema_for ! (crate :: index :: IndexPackage <'_ >) ; let dump = serde_json :: to_string_pretty (& schema) . unwrap () ; snapbox :: assert_data_eq ! (dump , snapbox :: file ! ("../index.schema.json") . raw ()) ; }
    };
}

dump_index_schema!();