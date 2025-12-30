// Generated macro for DisplayKind (enum)
macro_rules! Depcrate_displayDisplayKind {
() => {
// Module: crate::display
// Provides: {"DisplayKind"}
// Dependencies: {}
# [derive (Clone , Copy)] pub enum DisplayKind { # [doc = " Display types for inlays, doc popups, autocompletion, etc..."] # [doc = " Showing `{unknown}` or not qualifying paths is fine here."] # [doc = " There's no reason for this to fail."] Diagnostics , # [doc = " Display types for inserting them in source files."] # [doc = " The generated code should compile, so paths need to be qualified."] SourceCode { target_module_id : ModuleId , allow_opaque : bool } , # [doc = " Only for test purpose to keep real types"] Test , }
};
}
