macro_rules! deps {
    () => {
        LocalDefMap!();
        DefWithBodyId!();
        Resolver!();
        DefDatabase!();
        ExprScope!();
        LocalModuleId!();
        Scope!();
        DefMap!();
        GenericDefId!();
        ModuleItemMap!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < 'db > Resolver < 'db > { fn push_scope (mut self , scope : Scope < 'db >) -> Resolver < 'db > { self . scopes . push (scope) ; self } fn push_generic_params_scope (self , db : & 'db dyn DefDatabase , def : GenericDefId ,) -> Resolver < 'db > { let params = db . generic_params (def) ; self . push_scope (Scope :: GenericParams { def , params }) } fn push_block_scope (self , def_map : & 'db DefMap , local_def_map : & 'db LocalDefMap , module_id : LocalModuleId ,) -> Resolver < 'db > { self . push_scope (Scope :: BlockScope (ModuleItemMap { def_map , local_def_map , module_id })) } fn push_expr_scope (self , owner : DefWithBodyId , expr_scopes : Arc < ExprScopes > , scope_id : ScopeId ,) -> Resolver < 'db > { self . push_scope (Scope :: ExprScope (ExprScope { owner , expr_scopes , scope_id })) } }
    };
}

impl_320!()