macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'tcx > ScopeResolutionVisitor < 'tcx > { # [doc = " Records the current parent (if any) as the parent of `child_scope`."] fn record_child_scope (& mut self , child_scope : Scope) { let parent = self . cx . parent ; self . scope_tree . record_scope_parent (child_scope , parent) ; } # [doc = " Records the current parent (if any) as the parent of `child_scope`,"] # [doc = " and sets `child_scope` as the new current parent."] fn enter_scope (& mut self , child_scope : Scope) { self . record_child_scope (child_scope) ; self . cx . parent = Some (child_scope) ; } fn enter_node_scope_with_dtor (& mut self , id : hir :: ItemLocalId , terminating : bool) { if terminating { self . enter_scope (Scope { local_id : id , data : ScopeData :: Destruction }) ; } self . enter_scope (Scope { local_id : id , data : ScopeData :: Node }) ; } fn enter_body (& mut self , hir_id : hir :: HirId , f : impl FnOnce (& mut Self)) { let outer_cx = self . cx ; self . enter_scope (Scope { local_id : hir_id . local_id , data : ScopeData :: CallSite }) ; self . enter_scope (Scope { local_id : hir_id . local_id , data : ScopeData :: Arguments }) ; f (self) ; self . cx = outer_cx ; } }
    };
}

impl_88!();