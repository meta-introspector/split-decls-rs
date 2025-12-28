macro_rules! deps {
    () => {
        ToSql!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl Sealed for & [& dyn ToSql] { }
    };
}

impl_165!();