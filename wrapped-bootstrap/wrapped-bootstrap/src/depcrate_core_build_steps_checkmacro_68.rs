// Generated macro for macro_68 (macro)
macro_rules! Depcrate_core_build_steps_checkmacro_68 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"macro_68"}
// Dependencies: {}
tool_check_step ! (Compiletest { path : "src/tools/compiletest" , mode : | builder : & Builder <'_ >| if builder . config . compiletest_use_stage0_libtest { Mode :: ToolBootstrap } else { Mode :: ToolStd } , allow_features : COMPILETEST_ALLOW_FEATURES , default : false , }) ;
};
}
