macro_rules! record_rvalue_scope {
    () => {
        fn record_rvalue_scope (rvalue_scopes : & mut RvalueScopes , expr : & hir :: Expr < '_ > , candidate : & RvalueCandidate ,) { debug ! ("resolve_rvalue_scope(expr={expr:?}, candidate={candidate:?})") ; record_rvalue_scope_rec (rvalue_scopes , expr , candidate . lifetime , candidate . compat) }
    };
}

record_rvalue_scope!()