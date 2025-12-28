macro_rules! deps {
    () => {
        Label!();
        LabelValue!();
    };
}

macro_rules! explicit_label {
    () => {
        deps!();
        fn explicit_label (explicit : LabelValue) -> Label { match explicit { LabelValue :: Const (explicit) => Label :: Const (quote ! (# explicit)) , LabelValue :: Ident (explicit) => Label :: Ident (explicit) , } }
    };
}

explicit_label!();