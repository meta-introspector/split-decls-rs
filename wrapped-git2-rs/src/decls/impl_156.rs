macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Iter < 'a > { fn next_back (& mut self) -> Option < Option < & 'a str > > { self . range . next_back () . map (| i | self . arr . get (i)) } }
    };
}

impl_156!()