macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl error :: Error for Error { fn description (& self) -> & str { & self . message } }
    };
}

impl_484!()