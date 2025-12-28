macro_rules! deps {
    () => {
        FluentValue!();
        FluentNumber!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl From < FluentNumber > for FluentValue < '_ > { fn from (input : FluentNumber) -> Self { FluentValue :: Number (input) } }
    };
}

impl_81!();