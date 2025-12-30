// Generated macro for DEFAULT_SETTINGS (static)
macro_rules! Depcrate_settingsDEFAULT_SETTINGS {
() => {
// Module: crate::settings
// Provides: {"DEFAULT_SETTINGS"}
// Dependencies: {}
static DEFAULT_SETTINGS : Lazy < Arc < ActualSettings > > = Lazy :: new (| | { Arc :: new (ActualSettings { sort_maps : false , snapshot_path : "snapshots" . into () , snapshot_suffix : "" . into () , input_file : None , description : None , info : None , omit_expression : false , prepend_module_to_snapshot : true , # [cfg (feature = "redactions")] redactions : Redactions :: default () , # [cfg (feature = "filters")] filters : Filters :: default () , # [cfg (feature = "glob")] allow_empty_glob : false , }) }) ;
};
}
