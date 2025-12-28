macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'h , 's > Split < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8]) -> Split < 'h , 's > { let finder = haystack . find_iter (splitter) ; Split { finder , last : 0 , done : false } } }
    };
}

impl_89!();