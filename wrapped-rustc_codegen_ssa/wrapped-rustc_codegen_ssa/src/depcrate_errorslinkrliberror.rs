// Generated macro for LinkRlibError (enum)
macro_rules! Depcrate_errorsLinkRlibError {
() => {
// Module: crate::errors
// Provides: {"LinkRlibError"}
// Dependencies: {}
# [derive (Diagnostic)] pub enum LinkRlibError { # [diag (codegen_ssa_rlib_missing_format)] MissingFormat , # [diag (codegen_ssa_rlib_only_rmeta_found)] OnlyRmetaFound { crate_name : Symbol } , # [diag (codegen_ssa_rlib_not_found)] NotFound { crate_name : Symbol } , # [diag (codegen_ssa_rlib_incompatible_dependency_formats)] IncompatibleDependencyFormats { ty1 : String , ty2 : String , list1 : String , list2 : String } , }
};
}
