// Generated macro for find_binding (function)
macro_rules! Depcrate_redundant_localsfind_binding {
() => {
// Module: crate::redundant_locals
// Provides: {"find_binding"}
// Dependencies: {}
# [doc = " Find the annotation of a binding introduced by a pattern, or `None` if it's not introduced."] fn find_binding (pat : & Pat < '_ > , name : Ident) -> Option < BindingMode > { let mut ret = None ; pat . each_binding_or_first (& mut | annotation , _ , _ , ident | { if ident == name { ret = Some (annotation) ; } }) ; ret }
};
}
