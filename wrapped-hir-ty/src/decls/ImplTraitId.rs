macro_rules! ImplTraitId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] pub enum ImplTraitId < 'db > { ReturnTypeImplTrait (hir_def :: FunctionId , next_solver :: ImplTraitIdx < 'db >) , TypeAliasImplTrait (hir_def :: TypeAliasId , next_solver :: ImplTraitIdx < 'db >) , }
    };
}

ImplTraitId!()