// Generated macro for ScopeExitGuard (struct)
macro_rules! Depcrate_arrayvecScopeExitGuard {
() => {
// Module: crate::arrayvec
// Provides: {"ScopeExitGuard"}
// Dependencies: {}
struct ScopeExitGuard < T , Data , F > where F : FnMut (& Data , & mut T) , { value : T , data : Data , f : F , }
};
}
