macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl From < ModuleDef > for Definition { fn from (def : ModuleDef) -> Self { match def { ModuleDef :: Module (it) => Definition :: Module (it) , ModuleDef :: Function (it) => Definition :: Function (it) , ModuleDef :: Adt (it) => Definition :: Adt (it) , ModuleDef :: Variant (it) => Definition :: Variant (it) , ModuleDef :: Const (it) => Definition :: Const (it) , ModuleDef :: Static (it) => Definition :: Static (it) , ModuleDef :: Trait (it) => Definition :: Trait (it) , ModuleDef :: TypeAlias (it) => Definition :: TypeAlias (it) , ModuleDef :: Macro (it) => Definition :: Macro (it) , ModuleDef :: BuiltinType (it) => Definition :: BuiltinType (it) , } } }
    };
}

impl_42!()