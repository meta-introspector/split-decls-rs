macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! simplified_type_module {
    () => {
        deps!();
        pub fn simplified_type_module (db : & dyn HirDatabase , ty : & SimplifiedType) -> Option < ModuleId > { match ty . def () ? { SolverDefId :: AdtId (id) => Some (id . module (db)) , SolverDefId :: TypeAliasId (id) => Some (id . module (db)) , SolverDefId :: TraitId (id) => Some (id . module (db)) , _ => None , } }
    };
}

simplified_type_module!();