macro_rules! deps {
    () => {
        ParamsFromIter!();
        ToSql!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < I > Sealed for ParamsFromIter < I > where I : IntoIterator , I :: Item : ToSql , { }
    };
}

impl_193!();