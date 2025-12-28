macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_pat {
    () => {
        deps!();
        # [tracing :: instrument (level = "debug" , skip (visitor))] fn resolve_pat < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , pat : & 'tcx hir :: Pat < 'tcx >) { if let PatKind :: Binding (..) = pat . kind { record_var_lifetime (visitor , pat . hir_id . local_id) ; } intravisit :: walk_pat (visitor , pat) ; }
    };
}

resolve_pat!()