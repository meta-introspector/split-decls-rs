macro_rules! deps {
    () => {
        AssocItem!();
        Function!();
        Const!();
        TypeAlias!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl From < AssocItem > for GenericDefId { fn from (item : AssocItem) -> Self { match item { AssocItem :: Function (f) => f . id . into () , AssocItem :: Const (c) => c . id . into () , AssocItem :: TypeAlias (t) => t . id . into () , } } }
    };
}

impl_38!();