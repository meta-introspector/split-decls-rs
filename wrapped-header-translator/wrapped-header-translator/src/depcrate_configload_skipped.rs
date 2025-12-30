// Generated macro for load_skipped (function)
macro_rules! Depcrate_configload_skipped {
() => {
// Module: crate::config
// Provides: {"load_skipped"}
// Dependencies: {}
pub fn load_skipped () -> Result < BTreeMap < String , String > , Box < dyn Error + Send + Sync > > { let path = Path :: new (env ! ("CARGO_MANIFEST_DIR")) . join ("configs") . join ("skipped.toml") ; Ok (toml :: from_str (& fs :: read_to_string (path) ?) ?) }
};
}
