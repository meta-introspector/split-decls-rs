// Generated macro for FlycheckMessage (enum)
macro_rules! Depcrate_flycheckFlycheckMessage {
() => {
// Module: crate::flycheck
// Provides: {"FlycheckMessage"}
// Dependencies: {}
pub (crate) enum FlycheckMessage { # [doc = " Request adding a diagnostic with fixes included to a file"] AddDiagnostic { id : usize , generation : DiagnosticsGeneration , workspace_root : Arc < AbsPathBuf > , diagnostic : Diagnostic , package_id : Option < Arc < PackageId > > , } , # [doc = " Request clearing all outdated diagnostics."] ClearDiagnostics { id : usize , kind : ClearDiagnosticsKind } , # [doc = " Request check progress notification to client"] Progress { # [doc = " Flycheck instance ID"] id : usize , progress : Progress , } , }
};
}
