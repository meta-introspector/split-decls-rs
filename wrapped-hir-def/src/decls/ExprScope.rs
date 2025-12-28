macro_rules! deps {
    () => {
        DefWithBodyId!();
    };
}

macro_rules! ExprScope {
    () => {
        deps!();
        # [derive (Clone)] struct ExprScope { owner : DefWithBodyId , expr_scopes : Arc < ExprScopes > , scope_id : ScopeId , }
    };
}

ExprScope!();