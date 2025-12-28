macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Bytes < 'a > { # [inline] fn next_back (& mut self) -> Option < u8 > { self . it . next_back () . copied () } }
    };
}

impl_79!()