macro_rules! test {
    () => {
        # [cfg (test)] mod test { use semver :: Version ; # [derive (Debug , serde :: Deserialize)] struct BareVersion (# [serde (deserialize_with = "super::deserialize_rust_version")] Option < semver :: Version > ,) ; fn bare_version (str : & str) -> Version { serde_json :: from_str :: < BareVersion > (& format ! (r#""{}""# , str)) . unwrap () . 0 . unwrap () } fn bare_version_err (str : & str) -> String { serde_json :: from_str :: < BareVersion > (& format ! (r#""{}""# , str)) . unwrap_err () . to_string () } # [test] fn test_deserialize_rust_version () { assert_eq ! (bare_version ("1.2") , Version :: new (1 , 2 , 0)) ; assert_eq ! (bare_version ("1.2.0") , Version :: new (1 , 2 , 0)) ; assert_eq ! (bare_version_err ("1.2.0-alpha") , "pre-release identifiers are not supported in rust-version") ; assert_eq ! (bare_version_err ("1.2.0+123") , "build metadata is not supported in rust-version") ; } # [test] fn package_name_eq () { let my_package_name = super :: PackageName :: new ("my_package") ; assert_eq ! (my_package_name , "my_package") ; } }
    };
}

test!();