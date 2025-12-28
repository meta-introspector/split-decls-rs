macro_rules! deps {
    () => {
        ToSql!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Sealed for [& (dyn ToSql + Send + Sync) ; 0] { }
    };
}

impl_163!();