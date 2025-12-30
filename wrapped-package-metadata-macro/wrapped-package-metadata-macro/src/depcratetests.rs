// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , std :: str :: FromStr } ; # [test] fn package_metadata_string () { let copyright = "Copyright (c) 2024 ACME Inc." ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata]
            copyright = "{copyright}"
        "#)) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "copyright") . as_str () . unwrap () , copyright) ; } # [test] fn package_metadata_nested () { let program_id = "11111111111111111111111111111111" ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata.solana]
            program-id = "{program_id}"
        "#)) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "solana.program-id") . as_str () . unwrap () , program_id) ; } # [test] fn package_metadata_bool () { let manifest = toml :: from_str (r#"
            [package.metadata]
            is-ok = true
        "# ,) . unwrap () ; assert ! (package_metadata_value (& manifest , "is-ok") . as_bool () . unwrap ()) ; } # [test] fn package_metadata_int () { let number = 123 ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata]
            number = {number}
        "#)) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "number") . as_integer () . unwrap () , number) ; } # [test] fn package_metadata_float () { let float = 123.456 ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata]
            float = {float}
        "#)) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "float") . as_float () . unwrap () , float) ; } # [test] fn package_metadata_array () { let array = ["1" , "2" , "3"] ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata]
            array = {array:?}
        "#)) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "array") . as_array () . unwrap () . iter () . map (| x | x . as_str () . unwrap ()) . collect ::< Vec < _ >> () , array) ; } # [test] fn package_metadata_datetime () { let datetime = "1979-05-27T07:32:00Z" ; let manifest = toml :: from_str (& format ! (r#"
            [package.metadata]
            datetime = {datetime}
        "#)) . unwrap () ; let toml_datetime = toml :: value :: Datetime :: from_str (datetime) . unwrap () ; assert_eq ! (package_metadata_value (& manifest , "datetime") . as_datetime () . unwrap () , & toml_datetime) ; } }
};
}
