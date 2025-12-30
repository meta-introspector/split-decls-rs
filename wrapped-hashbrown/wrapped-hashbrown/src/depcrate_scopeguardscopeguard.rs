// Generated macro for ScopeGuard (struct)
macro_rules! Depcrate_scopeguardScopeGuard {
() => {
// Module: crate::scopeguard
// Provides: {"ScopeGuard"}
// Dependencies: {}
pub struct ScopeGuard < T , F > where F : FnMut (& mut T) , { dropfn : F , value : T , }
};
}
