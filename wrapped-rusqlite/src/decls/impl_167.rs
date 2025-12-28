macro_rules! deps {
    () => {
        BindIndex!();
        ToSql!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < S : BindIndex , T : ToSql > Sealed for & [(S , T)] { }
    };
}

impl_167!();