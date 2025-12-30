// Generated macro for panic_println (macro)
macro_rules! Depcrate_macrospanic_println {
() => {
// Module: crate::macros
// Provides: {"panic_println"}
// Dependencies: {}
# [cfg (not (target_os = "none"))] # [macro_export] macro_rules ! panic_println { ($ ($ arg : tt) *) => { println ! ($ ($ arg) *) ; } ; }
};
}
