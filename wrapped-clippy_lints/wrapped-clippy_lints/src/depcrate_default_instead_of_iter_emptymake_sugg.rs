// Generated macro for make_sugg (function)
macro_rules! Depcrate_default_instead_of_iter_emptymake_sugg {
() => {
// Module: crate::default_instead_of_iter_empty
// Provides: {"make_sugg"}
// Dependencies: {}
fn make_sugg (cx : & LateContext < '_ > , ty_path : & QPath < '_ > , ctxt : SyntaxContext , applicability : & mut Applicability , path : & str ,) -> String { if let Some (last) = last_path_segment (ty_path) . args && let Some (iter_ty) = last . args . iter () . find_map (| arg | match arg { GenericArg :: Type (ty) => Some (ty) , _ => None , }) { format ! ("{path}::<{}>()" , snippet_with_context (cx , iter_ty . span , ctxt , ".." , applicability) . 0) } else { format ! ("{path}()") } }
};
}
