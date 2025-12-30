// Generated macro for guard (function)
macro_rules! Depcrate_scopeguardguard {
() => {
// Module: crate::scopeguard
// Provides: {"guard"}
// Dependencies: {}
# [inline] pub fn guard < T , F > (value : T , dropfn : F) -> ScopeGuard < T , F > where F : FnMut (& mut T) , { ScopeGuard { dropfn , value } }
};
}
