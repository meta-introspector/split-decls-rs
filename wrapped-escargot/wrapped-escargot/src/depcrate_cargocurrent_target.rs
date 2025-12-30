// Generated macro for CURRENT_TARGET (const)
macro_rules! Depcrate_cargoCURRENT_TARGET {
() => {
// Module: crate::cargo
// Provides: {"CURRENT_TARGET"}
// Dependencies: {}
# [doc = " The current process' target triplet."] pub const CURRENT_TARGET : & str = include_str ! (concat ! (env ! ("OUT_DIR") , "/current_target.txt")) ;
};
}
