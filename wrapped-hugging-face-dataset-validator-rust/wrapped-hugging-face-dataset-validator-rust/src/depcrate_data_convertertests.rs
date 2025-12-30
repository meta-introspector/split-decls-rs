// Generated macro for tests (module)
macro_rules! Depcrate_data_convertertests {
() => {
// Module: crate::data_converter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: path :: Path ; # [test] fn test_data_converter () { let base_path = "/home/mdupont/2025/08/07/solfunmeme-index" ; if ! Path :: new (base_path) . exists () { println ! ("Skipping test - dataset not found at {}" , base_path) ; return ; } let converter = DataConverter :: new (base_path) . unwrap () ; let temp_file = "/tmp/test_export.jsonl" ; let result = converter . export_character_to_jsonl ("a" , temp_file) ; assert ! (result . is_ok ()) ; let count = result . unwrap () ; assert ! (count > 0) ; let _ = fs :: remove_file (temp_file) ; } }
};
}
