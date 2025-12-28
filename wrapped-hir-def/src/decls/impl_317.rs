macro_rules! deps {
    () => {
        AdtId!();
        ModuleDefId!();
        GenericParamId!();
        DefDatabase!();
        GenericDefId!();
        LifetimeParamId!();
        TypeOrConstParamId!();
        ExprScope!();
        Scope!();
        ScopeDef!();
        TypeParamId!();
        ConstParamId!();
        ScopeNames!();
        Label!();
        MacroId!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < 'db > Scope < 'db > { fn process_names (& self , acc : & mut ScopeNames , db : & 'db dyn DefDatabase) { match self { Scope :: BlockScope (m) => { m . def_map [m . module_id] . scope . entries () . for_each (| (name , def) | { acc . add_per_ns (name , def) ; }) ; m . def_map [m . module_id] . scope . legacy_macros () . for_each (| (name , macs) | { macs . iter () . for_each (| & mac | { acc . add (name , ScopeDef :: ModuleDef (ModuleDefId :: MacroId (mac))) ; }) }) ; } & Scope :: GenericParams { ref params , def : parent } => { if let GenericDefId :: ImplId (impl_) = parent { acc . add (& Name :: new_symbol_root (sym :: Self_) , ScopeDef :: ImplSelfType (impl_)) ; } else if let GenericDefId :: AdtId (adt) = parent { acc . add (& Name :: new_symbol_root (sym :: Self_) , ScopeDef :: AdtSelfType (adt)) ; } for (local_id , param) in params . iter_type_or_consts () { if let Some (name) = & param . name () { let id = TypeOrConstParamId { parent , local_id } ; let data = & db . generic_params (parent) [local_id] ; acc . add (name , ScopeDef :: GenericParam (match data { TypeOrConstParamData :: TypeParamData (_) => { GenericParamId :: TypeParamId (TypeParamId :: from_unchecked (id)) } TypeOrConstParamData :: ConstParamData (_) => { GenericParamId :: ConstParamId (ConstParamId :: from_unchecked (id)) } }) ,) ; } } for (local_id , param) in params . iter_lt () { let id = LifetimeParamId { parent , local_id } ; acc . add (& param . name , ScopeDef :: GenericParam (id . into ())) } } Scope :: ExprScope (scope) => { if let Some ((label , name)) = scope . expr_scopes . label (scope . scope_id) { acc . add (& name , ScopeDef :: Label (label)) } scope . expr_scopes . entries (scope . scope_id) . iter () . for_each (| e | { acc . add_local (e . name () , e . binding ()) ; }) ; } Scope :: MacroDefScope (_) => { } } } }
    };
}

impl_317!();