macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Lines < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { Some (trim_last_terminator (self . it . next_back () ?)) } }
    };
}

impl_103!()