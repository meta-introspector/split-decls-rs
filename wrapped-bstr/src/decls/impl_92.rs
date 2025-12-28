macro_rules! deps {
    () => {
        SplitReverse!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'h , 's > SplitReverse < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8]) -> SplitReverse < 'h , 's > { let finder = haystack . rfind_iter (splitter) ; SplitReverse { finder , last : haystack . len () , done : false } } }
    };
}

impl_92!();