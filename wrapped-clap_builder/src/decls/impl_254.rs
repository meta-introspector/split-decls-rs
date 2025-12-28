macro_rules! deps {
    () => {
        ValueParser!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl From < _AnonymousValueParser > for ValueParser { fn from (p : _AnonymousValueParser) -> Self { p . 0 } }
    };
}

impl_254!();