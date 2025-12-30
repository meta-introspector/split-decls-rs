// Generated macro for macro_270 (macro)
macro_rules! Depcrate_configmacro_270 {
() => {
// Module: crate::config
// Provides: {"macro_270"}
// Dependencies: {}
config_data ! { # [doc = " Configs that only make sense when they are set by a client. As such they can only be defined"] # [doc = " by setting them using client's settings (e.g `settings.json` on VS Code)."] client : struct ClientDefaultConfigData <- ClientConfigInput -> { # [doc = " Controls file watching implementation."] files_watcher : FilesWatcherDef = FilesWatcherDef :: Client , } }
};
}
