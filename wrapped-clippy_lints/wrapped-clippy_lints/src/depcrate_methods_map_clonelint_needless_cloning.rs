// Generated macro for lint_needless_cloning (function)
macro_rules! Depcrate_methods_map_clonelint_needless_cloning {
() => {
// Module: crate::methods::map_clone
// Provides: {"lint_needless_cloning"}
// Dependencies: {}
fn lint_needless_cloning (cx : & LateContext < '_ > , root : Span , receiver : Span) { span_lint_and_sugg (cx , MAP_CLONE , root . trim_start (receiver) . unwrap () , "you are needlessly cloning iterator elements" , "remove the `map` call" , String :: new () , Applicability :: MachineApplicable ,) ; }
};
}
