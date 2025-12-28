macro_rules! deps {
    () => {
        WindowsError!();
        Error!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl core :: error :: Error for WindowsError { }
    };
}

impl_74!()