// Generated macro for macro_435 (macro)
macro_rules! Depcrate_common_lazymacro_435 {
() => {
// Module: crate::common::lazy
// Provides: {"macro_435"}
// Dependencies: {}
pin_project ! { # [project = InnerProj] # [project_replace = InnerProjReplace] enum Inner < F , R > { Init { func : F } , Fut { # [pin] fut : R } , Empty , } }
};
}
