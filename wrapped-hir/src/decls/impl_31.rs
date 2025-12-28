macro_rules! deps {
    () => {
        TypeAlias!();
        Static!();
        Adt!();
        GenericDef!();
        Const!();
        Impl!();
        Trait!();
        Function!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < GenericDef > for GenericDefId { fn from (def : GenericDef) -> Self { match def { GenericDef :: Function (it) => GenericDefId :: FunctionId (it . id) , GenericDef :: Adt (it) => GenericDefId :: AdtId (it . into ()) , GenericDef :: Trait (it) => GenericDefId :: TraitId (it . id) , GenericDef :: TypeAlias (it) => GenericDefId :: TypeAliasId (it . id) , GenericDef :: Impl (it) => GenericDefId :: ImplId (it . id) , GenericDef :: Const (it) => GenericDefId :: ConstId (it . id) , GenericDef :: Static (it) => GenericDefId :: StaticId (it . id) , } } }
    };
}

impl_31!()