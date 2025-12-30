// Generated macro for IS_ALWAYS_LOCK_FREE (const)
macro_rules! Depcrate_imp_interruptIS_ALWAYS_LOCK_FREE {
() => {
// Module: crate::imp::interrupt
// Provides: {"IS_ALWAYS_LOCK_FREE"}
// Dependencies: {}
# [cfg (not (feature = "critical-section"))] const IS_ALWAYS_LOCK_FREE : bool = true ;
};
}
