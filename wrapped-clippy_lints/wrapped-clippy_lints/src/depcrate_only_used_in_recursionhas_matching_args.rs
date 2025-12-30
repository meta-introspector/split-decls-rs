// Generated macro for has_matching_args (function)
macro_rules! Depcrate_only_used_in_recursionhas_matching_args {
() => {
// Module: crate::only_used_in_recursion
// Provides: {"has_matching_args"}
// Dependencies: {}
fn has_matching_args (kind : FnKind , args : GenericArgsRef < '_ >) -> bool { match kind { FnKind :: Fn => true , FnKind :: TraitFn => args . iter () . enumerate () . all (| (idx , subst) | match subst . kind () { GenericArgKind :: Lifetime (_) => true , GenericArgKind :: Type (ty) => matches ! (* ty . kind () , ty :: Param (ty) if ty . index as usize == idx) , GenericArgKind :: Const (c) => matches ! (c . kind () , ConstKind :: Param (c) if c . index as usize == idx) , }) , FnKind :: ImplTraitFn (expected_args) => std :: ptr :: from_ref (args) as usize == expected_args , } }
};
}
