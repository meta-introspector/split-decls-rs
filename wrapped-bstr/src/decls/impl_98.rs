macro_rules! deps {
    () => {
        SplitNReverse!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'h , 's > SplitNReverse < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8] , limit : usize ,) -> SplitNReverse < 'h , 's > { let split = haystack . rsplit_str (splitter) ; SplitNReverse { split , limit , count : 0 } } }
    };
}

impl_98!()