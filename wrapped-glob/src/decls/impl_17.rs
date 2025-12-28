macro_rules! deps {
    () => {
        PatternError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Error for PatternError { fn description (& self) -> & str { self . msg } }
    };
}

impl_17!();