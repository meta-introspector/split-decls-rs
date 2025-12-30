// Generated macro for test_parse_yaml_error (function)
macro_rules! Depcrate_snapshottest_parse_yaml_error {
() => {
// Module: crate::snapshot
// Provides: {"test_parse_yaml_error"}
// Dependencies: {}
# [test] fn test_parse_yaml_error () { use std :: env :: temp_dir ; let mut temp = temp_dir () ; temp . push ("bad.yaml") ; let mut f = fs :: File :: create (temp . clone ()) . unwrap () ; let invalid = r#"---
    This is invalid yaml:
     {
        {
    ---
    "# ; f . write_all (invalid . as_bytes ()) . unwrap () ; let error = format ! ("{}" , Snapshot :: from_file (temp . as_path ()) . unwrap_err ()) ; assert ! (error . contains ("Failed parsing the YAML from")) ; assert ! (error . contains ("bad.yaml")) ; }
};
}
