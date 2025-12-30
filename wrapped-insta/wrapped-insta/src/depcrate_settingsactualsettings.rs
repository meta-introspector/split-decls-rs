// Generated macro for ActualSettings (struct)
macro_rules! Depcrate_settingsActualSettings {
() => {
// Module: crate::settings
// Provides: {"ActualSettings"}
// Dependencies: {}
# [derive (Clone)] # [doc (hidden)] pub struct ActualSettings { pub sort_maps : bool , pub snapshot_path : PathBuf , pub snapshot_suffix : String , pub input_file : Option < PathBuf > , pub description : Option < String > , pub info : Option < Content > , pub omit_expression : bool , pub prepend_module_to_snapshot : bool , # [cfg (feature = "redactions")] pub redactions : Redactions , # [cfg (feature = "filters")] pub filters : Filters , # [cfg (feature = "glob")] pub allow_empty_glob : bool , }
};
}
