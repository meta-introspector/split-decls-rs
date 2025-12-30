// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl DepCache { fn new () -> Self { let cache_dir = std :: path :: PathBuf :: from ("../bootstrap3-incremental/dep-cache") ; fs :: create_dir_all (& cache_dir) . unwrap () ; Self { nodes : HashMap :: new () , cache_dir , } } fn get_or_compute (& mut self , path : & std :: path :: Path) -> Result < String , Box < dyn std :: error :: Error > > { let content = fs :: read_to_string (path) ? ; let content_hash = hash_content (& content) ; let path_str = path . to_string_lossy () . to_string () ; let id = format ! ("dep-{:x}" , hash_content (& path_str)) ; if let Some (node) = self . nodes . get (& id) { if node . content_hash == content_hash { return Ok (id) ; } } let tokens = extract_tokens_from_content (& content) ? ; let node = DepNode { id : id . clone () , hash : content_hash , path : path . to_path_buf () , content_hash , tokens , resolved_deps : Vec :: new () , } ; self . save_node_to_cache (& node) ? ; self . nodes . insert (id . clone () , node) ; Ok (id) } fn save_node_to_cache (& self , node : & DepNode) -> Result < () , Box < dyn std :: error :: Error > > { let cache_file = self . cache_dir . join (format ! ("{}.json" , node . id)) ; let json = serde_json :: to_string_pretty (& CachedNode { id : node . id . clone () , hash : node . hash , path : node . path . to_string_lossy () . to_string () , content_hash : node . content_hash , tokens : node . tokens . iter () . cloned () . collect () , resolved_deps : node . resolved_deps . clone () , }) ? ; fs :: write (cache_file , json) ? ; Ok (()) } }
};
}
