macro_rules! deps {
    () => {
        ParseIntegerError!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl std :: error :: Error for ParseIntegerError { fn description (& self) -> & str { self . desc () } }
    };
}

impl_22!();