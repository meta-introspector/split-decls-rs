macro_rules! deps {
    () => {
        HasIterator!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl BitOr < Self > for HasIterator { type Output = Self ; fn bitor (self , _rhs : Self) -> Self { Self } }
    };
}

impl_153!();