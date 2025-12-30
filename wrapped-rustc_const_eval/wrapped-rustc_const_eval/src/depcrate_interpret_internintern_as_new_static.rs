// Generated macro for intern_as_new_static (function)
macro_rules! Depcrate_interpret_internintern_as_new_static {
() => {
// Module: crate::interpret::intern
// Provides: {"intern_as_new_static"}
// Dependencies: {}
# [doc = " Creates a new `DefId` and feeds all the right queries to make this `DefId`"] # [doc = " appear as if it were a user-written `static` (though it has no HIR)."] fn intern_as_new_static < 'tcx > (tcx : TyCtxtAt < 'tcx > , static_id : LocalDefId , alloc_id : AllocId , alloc : ConstAllocation < 'tcx > , disambiguator : & mut DisambiguatorState ,) { let feed = tcx . create_def (static_id , None , DefKind :: Static { safety : hir :: Safety :: Safe , mutability : alloc . 0 . mutability , nested : true } , Some (DefPathData :: NestedStatic) , disambiguator ,) ; tcx . set_nested_alloc_id_static (alloc_id , feed . def_id ()) ; if tcx . is_thread_local_static (static_id . into ()) { tcx . dcx () . emit_err (errors :: NestedStaticInThreadLocal { span : tcx . def_span (static_id) }) ; } feed . codegen_fn_attrs (CodegenFnAttrs :: new ()) ; feed . eval_static_initializer (Ok (alloc)) ; feed . generics_of (tcx . generics_of (static_id) . clone ()) ; feed . def_ident_span (tcx . def_ident_span (static_id)) ; feed . explicit_predicates_of (tcx . explicit_predicates_of (static_id)) ; feed . feed_hir () ; }
};
}
