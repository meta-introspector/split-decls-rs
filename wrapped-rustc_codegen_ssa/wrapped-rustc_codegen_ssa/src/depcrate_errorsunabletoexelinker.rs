// Generated macro for UnableToExeLinker (struct)
macro_rules! Depcrate_errorsUnableToExeLinker {
() => {
// Module: crate::errors
// Provides: {"UnableToExeLinker"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_unable_to_exe_linker)] # [note] # [note (codegen_ssa_command_note)] pub (crate) struct UnableToExeLinker { pub linker_path : PathBuf , pub error : Error , pub command_formatted : String , }
};
}
