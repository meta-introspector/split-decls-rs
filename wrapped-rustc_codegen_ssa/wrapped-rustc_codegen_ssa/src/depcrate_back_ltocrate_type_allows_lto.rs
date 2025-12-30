// Generated macro for crate_type_allows_lto (function)
macro_rules! Depcrate_back_ltocrate_type_allows_lto {
() => {
// Module: crate::back::lto
// Provides: {"crate_type_allows_lto"}
// Dependencies: {}
fn crate_type_allows_lto (crate_type : CrateType) -> bool { match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Staticlib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib => true , CrateType :: Rlib => false , } }
};
}
