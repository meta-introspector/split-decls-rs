// Generated macro for fixme (macro)
macro_rules! Depcratefixme {
() => {
// Module: crate
// Provides: {"fixme"}
// Dependencies: {}
# [macro_export] macro_rules ! fixme { ($ msg : literal) => { { use patch_build_rs_macros :: extract ; extract ! ($ msg) ; compile_error ! (concat ! ("FIXME: " , $ msg)) ; } } ; }
};
}
