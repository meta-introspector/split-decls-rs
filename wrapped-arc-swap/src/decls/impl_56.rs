macro_rules! deps {
    () => {
        Debt!();
        Slots!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Slots { type Item = & 'a Debt ; type IntoIter = Iter < 'a , Debt > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . iter () } }
    };
}

impl_56!();