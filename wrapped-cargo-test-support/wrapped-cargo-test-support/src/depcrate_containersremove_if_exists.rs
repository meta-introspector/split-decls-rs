// Generated macro for remove_if_exists (function)
macro_rules! Depcrate_containersremove_if_exists {
() => {
// Module: crate::containers
// Provides: {"remove_if_exists"}
// Dependencies: {}
fn remove_if_exists (name : & str) { if let Err (e) = Command :: new ("docker") . args (& ["container" , "rm" , "--force" , name]) . output () { panic ! ("failed to run docker: {e}") ; } }
};
}
