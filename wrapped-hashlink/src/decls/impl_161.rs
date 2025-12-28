macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > { # [inline] fn next_back (& mut self) -> Option < & 'a T > { self . iter . next_back () } }
    };
}

impl_161!()