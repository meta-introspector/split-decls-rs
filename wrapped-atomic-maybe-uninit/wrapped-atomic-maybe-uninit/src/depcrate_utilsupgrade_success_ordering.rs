// Generated macro for upgrade_success_ordering (function)
macro_rules! Depcrate_utilsupgrade_success_ordering {
() => {
// Module: crate::utils
// Provides: {"upgrade_success_ordering"}
// Dependencies: {}
# [allow (dead_code)] # [inline] pub (crate) fn upgrade_success_ordering (success : Ordering , failure : Ordering) -> Ordering { match (success , failure) { (Ordering :: Relaxed , Ordering :: Acquire) => Ordering :: Acquire , (Ordering :: Release , Ordering :: Acquire) => Ordering :: AcqRel , (_ , Ordering :: SeqCst) => Ordering :: SeqCst , _ => success , } }
};
}
