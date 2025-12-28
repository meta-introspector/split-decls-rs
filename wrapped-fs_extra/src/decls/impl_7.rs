macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl StdError for Error { fn description (& self) -> & str { self . kind . as_str () } }
    };
}

impl_7!()