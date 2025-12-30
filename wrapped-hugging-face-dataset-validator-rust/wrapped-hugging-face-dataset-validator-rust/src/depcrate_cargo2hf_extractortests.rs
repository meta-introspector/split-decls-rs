// Generated macro for tests (module)
macro_rules! Depcrate_cargo2hf_extractortests {
() => {
// Module: crate::cargo2hf_extractor
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use tempfile :: TempDir ; use std :: fs ; # [test] fn test_cargo2hf_extractor_creation () { let extractor = Cargo2HfExtractor :: new () ; assert ! (extractor . is_ok ()) ; } # [test] fn test_project_metadata_extraction () { let temp_dir = TempDir :: new () . unwrap () ; let cargo_toml = temp_dir . path () . join ("Cargo.toml") ; fs :: write (& cargo_toml , r#"
[package]
name = "test-project"
version = "0.1.0"
description = "A test project"
authors = ["Test Author <test@example.com>"]
license = "MIT"
"#) . unwrap () ; let mut extractor = Cargo2HfExtractor :: new () . unwrap () ; let records = extractor . extract_project_metadata (temp_dir . path ()) . unwrap () ; assert_eq ! (records . len () , 1) ; assert_eq ! (records [0] . project_name , "test-project") ; assert_eq ! (records [0] . project_version , "0.1.0") ; assert_eq ! (records [0] . description , Some ("A test project" . to_string ())) ; assert_eq ! (records [0] . license , Some ("MIT" . to_string ())) ; } }
};
}
