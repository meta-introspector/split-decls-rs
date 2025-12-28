macro_rules! deps {
    () => {
        JacobiSymbol!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Eq for JacobiSymbol { }
    };
}

impl_129!();