// Generated macro for lint_explicit_closure (function)
macro_rules! Depcrate_methods_map_clonelint_explicit_closure {
() => {
// Module: crate::methods::map_clone
// Provides: {"lint_explicit_closure"}
// Dependencies: {}
fn lint_explicit_closure (cx : & LateContext < '_ > , replace : Span , root : Span , is_copy : bool , msrv : Msrv) { let mut applicability = Applicability :: MachineApplicable ; let (message , sugg_method) = if is_copy && msrv . meets (cx , msrvs :: ITERATOR_COPIED) { ("you are using an explicit closure for copying elements" , "copied") } else { ("you are using an explicit closure for cloning elements" , "cloned") } ; span_lint_and_sugg (cx , MAP_CLONE , replace , message , format ! ("consider calling the dedicated `{sugg_method}` method") , format ! ("{}.{sugg_method}()" , snippet_with_applicability (cx , root , ".." , & mut applicability) ,) , applicability ,) ; }
};
}
