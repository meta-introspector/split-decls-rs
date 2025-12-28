macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Iter < 'a > { # [inline] fn next_back (& mut self) -> Option < & 'a str > { self . inner . next_back () . map (| component | component . as_str ()) } }
    };
}

impl_54!()