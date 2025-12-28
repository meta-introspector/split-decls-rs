macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! record_var_lifetime {
    () => {
        deps!();
        # [doc = " Records the lifetime of a local variable as `cx.var_parent`"] fn record_var_lifetime (visitor : & mut ScopeResolutionVisitor < '_ > , var_id : hir :: ItemLocalId) { let (var_parent_scope , var_parent_compat) = visitor . cx . var_parent ; match var_parent_scope { None => { } Some (parent_scope) => visitor . scope_tree . record_var_scope (var_id , parent_scope) , } if let ScopeCompatibility :: FutureIncompatible { shortens_to } = var_parent_compat { visitor . scope_tree . record_future_incompatible_var_scope (var_id , shortens_to) ; } }
    };
}

record_var_lifetime!();