macro_rules! deps {
    () => {
        DlError!();
        Error!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl core :: error :: Error for DlError { }
    };
}

impl_69!();