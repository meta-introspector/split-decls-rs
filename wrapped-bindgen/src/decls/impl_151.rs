macro_rules! deps {
    () => {
        HasIterator!();
        ThereIsNoIteratorInRepetition!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl BitOr < ThereIsNoIteratorInRepetition > for HasIterator { type Output = Self ; fn bitor (self , _rhs : ThereIsNoIteratorInRepetition) -> Self { Self } }
    };
}

impl_151!();