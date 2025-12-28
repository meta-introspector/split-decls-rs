macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl From < GenericDef > for Definition { fn from (def : GenericDef) -> Self { match def { GenericDef :: Function (it) => it . into () , GenericDef :: Adt (it) => it . into () , GenericDef :: Trait (it) => it . into () , GenericDef :: TypeAlias (it) => it . into () , GenericDef :: Impl (it) => it . into () , GenericDef :: Const (it) => it . into () , GenericDef :: Static (it) => it . into () , } } }
    };
}

impl_46!();