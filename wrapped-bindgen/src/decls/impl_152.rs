macro_rules! deps {
    () => {
        HasIterator!();
        ThereIsNoIteratorInRepetition!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl BitOr < HasIterator > for ThereIsNoIteratorInRepetition { type Output = HasIterator ; fn bitor (self , _rhs : HasIterator) -> HasIterator { HasIterator } }
    };
}

impl_152!()