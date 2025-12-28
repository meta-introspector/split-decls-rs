macro_rules! deps {
    () => {
        ValueSource!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl ValueSource { pub (crate) fn is_explicit (self) -> bool { self != Self :: DefaultValue } }
    };
}

impl_506!()