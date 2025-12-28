macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < AssocItem > for Definition { fn from (assoc_item : AssocItem) -> Self { match assoc_item { AssocItem :: Function (it) => Definition :: Function (it) , AssocItem :: Const (it) => Definition :: Const (it) , AssocItem :: TypeAlias (it) => Definition :: TypeAlias (it) , } } }
    };
}

impl_40!()