// Generated macro for tests (module)
macro_rules! Depcrate_rust_analyzer_extractortests {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: fs ; use tempfile :: TempDir ; # [test] fn test_rust_analyzer_extractor_creation () { let extractor = RustAnalyzerExtractor :: new () ; assert ! (extractor . is_ok ()) ; } # [test] fn test_find_rust_files () { let temp_dir = TempDir :: new () . unwrap () ; let rust_file = temp_dir . path () . join ("test.rs") ; fs :: write (& rust_file , "fn main() {}") . unwrap () ; let extractor = RustAnalyzerExtractor :: new () . unwrap () ; let rust_files = extractor . find_rust_files (temp_dir . path ()) . unwrap () ; assert_eq ! (rust_files . len () , 1) ; assert_eq ! (rust_files [0] , rust_file) ; } # [test] fn test_extract_parsing_data () { let temp_dir = TempDir :: new () . unwrap () ; let rust_file = temp_dir . path () . join ("test.rs") ; fs :: write (& rust_file , "fn main() {\n    println!(\"Hello, world!\");\n}") . unwrap () ; let mut extractor = RustAnalyzerExtractor :: new () . unwrap () ; let records = extractor . extract_parsing_data (& rust_file) . unwrap () ; assert ! (! records . is_empty ()) ; assert_eq ! (records [0] . phase , "parsing") ; assert_eq ! (records [0] . element_type , "function") ; } # [test] fn test_element_type_detection () { let extractor = RustAnalyzerExtractor :: new () . unwrap () ; assert_eq ! (extractor . detect_element_type ("fn main() {") , "function") ; assert_eq ! (extractor . detect_element_type ("struct Point {") , "struct") ; assert_eq ! (extractor . detect_element_type ("enum Color {") , "enum") ; assert_eq ! (extractor . detect_element_type ("let x = 5;") , "variable") ; } }
};
}
