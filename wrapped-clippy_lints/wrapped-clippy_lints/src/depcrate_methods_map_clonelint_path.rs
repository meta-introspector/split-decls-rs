// Generated macro for lint_path (function)
macro_rules! Depcrate_methods_map_clonelint_path {
() => {
// Module: crate::methods::map_clone
// Provides: {"lint_path"}
// Dependencies: {}
fn lint_path (cx : & LateContext < '_ > , replace : Span , root : Span , is_copy : bool) { let mut applicability = Applicability :: MachineApplicable ; let replacement = if is_copy { "copied" } else { "cloned" } ; span_lint_and_sugg (cx , MAP_CLONE , replace , "you are explicitly cloning with `.map()`" , format ! ("consider calling the dedicated `{replacement}` method") , format ! ("{}.{replacement}()" , snippet_with_applicability (cx , root , ".." , & mut applicability) ,) , applicability ,) ; }
};
}
