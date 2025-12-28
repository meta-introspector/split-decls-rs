macro_rules! deps {
    () => {
        Resolver!();
        DefDatabase!();
        DefWithBodyId!();
    };
}

macro_rules! resolver_for_scope {
    () => {
        deps!();
        pub fn resolver_for_scope (db : & dyn DefDatabase , owner : DefWithBodyId , scope_id : Option < ScopeId > ,) -> Resolver < '_ > { let r = owner . resolver (db) ; let scopes = db . expr_scopes (owner) ; resolver_for_scope_ (db , scopes , scope_id , r , owner) }
    };
}

resolver_for_scope!()