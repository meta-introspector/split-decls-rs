macro_rules! deps {
    () => {
        DrainBytes!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for DrainBytes < 'a > { # [inline] fn next_back (& mut self) -> Option < u8 > { self . it . next_back () } }
    };
}

impl_122!()