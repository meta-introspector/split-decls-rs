// Generated macro for is_used (function)
macro_rules! Depcrate_utilsis_used {
() => {
// Module: crate::utils
// Provides: {"is_used"}
// Dependencies: {}
fn is_used (id : & Ident , references : & HashMap < Ident , HashSet < Ident > > , ends : & HashSet < Ident >) -> bool { let mut visited = Default :: default () ; _is_used (& mut visited , id , references , ends) }
};
}
