macro_rules! deps {
    () => {
        ToSql!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T : ToSql > Sealed for (T ,) { }
    };
}

impl_171!()