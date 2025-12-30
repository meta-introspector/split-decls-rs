// Generated macro for load_config (function)
macro_rules! Depcrate_configload_config {
() => {
// Module: crate::config
// Provides: {"load_config"}
// Dependencies: {}
pub fn load_config () -> Result < Config , Box < dyn Error + Send + Sync > > { let manifest_dir = Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; let workspace_dir = manifest_dir . parent () . unwrap () . parent () . unwrap () ; let _span = info_span ! ("loading configs") . entered () ; let mut libraries = BTreeMap :: default () ; for dir in fs :: read_dir (workspace_dir . join ("framework-crates")) ? { let dir = dir ? ; if ! dir . file_type () ? . is_dir () { continue ; } let path = dir . path () . join ("translation-config.toml") ; let config = LibraryConfig :: from_file (& path) . unwrap_or_else (| e | panic ! ("read {path:?} config: {e}")) ; assert_eq ! (* config . krate , * dir . file_name ()) ; libraries . insert (config . framework . to_string () , config) ; } let path = workspace_dir . join ("crates") . join ("block2") . join ("translation-config.toml") ; let objc = toml :: from_str (& fs :: read_to_string (path) ?) ? ; libraries . insert ("block" . to_string () , objc) ; let path = workspace_dir . join ("crates") . join ("objc2") . join ("translation-config.toml") ; let objc = toml :: from_str (& fs :: read_to_string (path) ?) ? ; libraries . insert ("ObjectiveC" . to_string () , objc) ; let path = workspace_dir . join ("crates") . join ("dispatch2") . join ("translation-config.toml") ; let objc = toml :: from_str (& fs :: read_to_string (path) ?) ? ; libraries . insert ("Dispatch" . to_string () , objc) ; Config :: new (libraries) }
};
}
