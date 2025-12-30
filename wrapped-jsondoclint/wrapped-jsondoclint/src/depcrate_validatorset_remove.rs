// Generated macro for set_remove (function)
macro_rules! Depcrate_validatorset_remove {
() => {
// Module: crate::validator
// Provides: {"set_remove"}
// Dependencies: {}
fn set_remove < T : Hash + Eq + Clone > (set : & mut HashSet < T >) -> Option < T > { if let Some (id) = set . iter () . next () { let id = id . clone () ; set . take (& id) } else { None } }
};
}
