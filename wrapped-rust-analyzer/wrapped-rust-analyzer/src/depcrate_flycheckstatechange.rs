// Generated macro for StateChange (enum)
macro_rules! Depcrate_flycheckStateChange {
() => {
// Module: crate::flycheck
// Provides: {"StateChange"}
// Dependencies: {}
enum StateChange { Restart { generation : DiagnosticsGeneration , scope : FlycheckScope , saved_file : Option < AbsPathBuf > , target : Option < Target > , } , Cancel , }
};
}
