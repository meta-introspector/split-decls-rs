// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cacheimpl_12 {
() => {
// Module: crate::cache
// Provides: {"impl_12"}
// Dependencies: {}
impl Cache { # [doc = " Create a new cache, used to read files only once and otherwise store their contents."] pub fn new (config : & Config) -> Cache { let root = Path :: new (& config . doc_dir) ; let filename = Path :: new (& config . template) . file_stem () . unwrap () . to_str () . unwrap () . replace ('-' , "_") ; let file_path = root . join (& Path :: with_extension (Path :: new (& filename) , "json")) ; let content = fs :: read_to_string (& file_path) . expect ("failed to read JSON file") ; Cache { value : serde_json :: from_str :: < Value > (& content) . expect ("failed to convert from JSON") , variables : HashMap :: from ([("FILE" . to_owned () , config . template . clone () . into ())]) , } } pub fn select (& self , path : & str) -> Vec < & Value > { jsonpath_rust :: query :: js_path_vals (path , & self . value) . unwrap () } }
};
}
