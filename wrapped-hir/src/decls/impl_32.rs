macro_rules! deps {
    () => {
        Impl!();
        GenericDef!();
        Adt!();
        Trait!();
        TypeAlias!();
        Function!();
        Const!();
        Static!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl From < GenericDefId > for GenericDef { fn from (def : GenericDefId) -> Self { match def { GenericDefId :: FunctionId (it) => GenericDef :: Function (it . into ()) , GenericDefId :: AdtId (it) => GenericDef :: Adt (it . into ()) , GenericDefId :: TraitId (it) => GenericDef :: Trait (it . into ()) , GenericDefId :: TypeAliasId (it) => GenericDef :: TypeAlias (it . into ()) , GenericDefId :: ImplId (it) => GenericDef :: Impl (it . into ()) , GenericDefId :: ConstId (it) => GenericDef :: Const (it . into ()) , GenericDefId :: StaticId (it) => GenericDef :: Static (it . into ()) , } } }
    };
}

impl_32!()