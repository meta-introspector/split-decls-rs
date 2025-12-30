// Generated macro for get_pgo_gen_path (function)
macro_rules! Depcrate_back_writeget_pgo_gen_path {
() => {
// Module: crate::back::write
// Provides: {"get_pgo_gen_path"}
// Dependencies: {}
fn get_pgo_gen_path (config : & ModuleConfig) -> Option < CString > { match config . pgo_gen { SwitchWithOptPath :: Enabled (ref opt_dir_path) => { let path = if let Some (dir_path) = opt_dir_path { dir_path . join ("default_%m.profraw") } else { PathBuf :: from ("default_%m.profraw") } ; Some (CString :: new (format ! ("{}" , path . display ())) . unwrap ()) } SwitchWithOptPath :: Disabled => None , } }
};
}
