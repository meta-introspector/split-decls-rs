// Generated macro for clone_ref (function)
macro_rules! Depcrate_utilclone_ref {
() => {
// Module: crate::util
// Provides: {"clone_ref"}
// Dependencies: {}
pub (crate) fn clone_ref < A > (r : Ref < A >) -> A where A : Clone , { Ref :: try_unwrap (r) . unwrap_or_else (| r | (* r) . clone ()) }
};
}
