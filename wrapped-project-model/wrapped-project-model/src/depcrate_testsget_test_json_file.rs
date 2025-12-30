// Generated macro for get_test_json_file (function)
macro_rules! Depcrate_testsget_test_json_file {
() => {
// Module: crate::tests
// Provides: {"get_test_json_file"}
// Dependencies: {}
fn get_test_json_file < T : DeserializeOwned > (file : & str) -> T { let file = get_test_path (file) ; let data = std :: fs :: read_to_string (file) . unwrap () ; let mut json = data . parse :: < serde_json :: Value > () . unwrap () ; fixup_paths (& mut json) ; return serde_json :: from_value (json) . unwrap () ; fn fixup_paths (val : & mut serde_json :: Value) { match val { serde_json :: Value :: String (s) => replace_root (s , true) , serde_json :: Value :: Array (vals) => vals . iter_mut () . for_each (fixup_paths) , serde_json :: Value :: Object (kvals) => kvals . values_mut () . for_each (fixup_paths) , serde_json :: Value :: Null | serde_json :: Value :: Bool (_) | serde_json :: Value :: Number (_) => { } } } }
};
}
