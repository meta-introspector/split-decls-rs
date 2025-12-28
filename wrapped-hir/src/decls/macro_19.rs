macro_rules! deps {
    () => {
        ConstParam!();
        Function!();
        Trait!();
        ExternBlock!();
        Crate!();
        Struct!();
        TypeParam!();
        TypeOrConstParam!();
        ExternCrateDecl!();
        TypeAlias!();
        Union!();
        Static!();
        LifetimeParam!();
        Enum!();
        Macro!();
        Module!();
        Const!();
        Impl!();
    };
}

macro_rules! macro_19 {
    () => {
        deps!();
        from_id ! [(base_db :: Crate , crate :: Crate) , (hir_def :: ModuleId , crate :: Module) , (hir_def :: StructId , crate :: Struct) , (hir_def :: UnionId , crate :: Union) , (hir_def :: EnumId , crate :: Enum) , (hir_def :: TypeAliasId , crate :: TypeAlias) , (hir_def :: TraitId , crate :: Trait) , (hir_def :: StaticId , crate :: Static) , (hir_def :: ConstId , crate :: Const) , (hir_def :: FunctionId , crate :: Function) , (hir_def :: ImplId , crate :: Impl) , (hir_def :: TypeOrConstParamId , crate :: TypeOrConstParam) , (hir_def :: TypeParamId , crate :: TypeParam) , (hir_def :: ConstParamId , crate :: ConstParam) , (hir_def :: LifetimeParamId , crate :: LifetimeParam) , (hir_def :: MacroId , crate :: Macro) , (hir_def :: ExternCrateId , crate :: ExternCrateDecl) , (hir_def :: ExternBlockId , crate :: ExternBlock) ,] ;
    };
}

macro_19!();