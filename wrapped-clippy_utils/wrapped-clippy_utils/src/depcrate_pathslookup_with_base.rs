// Generated macro for lookup_with_base (function)
macro_rules! Depcrate_pathslookup_with_base {
() => {
// Module: crate::paths
// Provides: {"lookup_with_base"}
// Dependencies: {}
# [doc = " Resolves a def path like `vec::Vec` with the base `std`."] fn lookup_with_base (tcx : TyCtxt < '_ > , mut base : DefId , ns : PathNS , mut path : & [Symbol] , out : & mut Vec < DefId >) { loop { match * path { [segment] => { out . extend (item_child_by_name (tcx , base , ns , segment)) ; let inherent_impl_children = tcx . inherent_impls (base) . iter () . filter_map (| & impl_def_id | item_child_by_name (tcx , impl_def_id , ns , segment)) ; out . extend (inherent_impl_children) ; return ; } , [segment , ref rest @ ..] => { path = rest ; let Some (child) = item_child_by_name (tcx , base , PathNS :: Type , segment) else { return ; } ; base = child ; } , [] => unreachable ! () , } } }
};
}
