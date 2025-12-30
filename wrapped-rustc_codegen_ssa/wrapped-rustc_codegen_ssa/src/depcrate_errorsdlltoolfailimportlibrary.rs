// Generated macro for DlltoolFailImportLibrary (struct)
macro_rules! Depcrate_errorsDlltoolFailImportLibrary {
() => {
// Module: crate::errors
// Provides: {"DlltoolFailImportLibrary"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_dlltool_fail_import_library)] pub (crate) struct DlltoolFailImportLibrary < 'a > { pub dlltool_path : Cow < 'a , str > , pub dlltool_args : String , pub stdout : Cow < 'a , str > , pub stderr : Cow < 'a , str > , }
};
}
