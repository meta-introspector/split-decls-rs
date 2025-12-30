// Generated macro for with (function)
macro_rules! Depcrate_imp_interruptwith {
() => {
// Module: crate::imp::interrupt
// Provides: {"with"}
// Dependencies: {}
# [cfg (not (feature = "critical-section"))] # [inline (always)] fn with < F , R > (f : F) -> R where F : FnOnce () -> R , { let state = arch :: disable () ; let r = f () ; unsafe { arch :: restore (state) } r }
};
}
