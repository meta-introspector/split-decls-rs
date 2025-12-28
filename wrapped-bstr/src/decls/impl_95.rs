macro_rules! deps {
    () => {
        SplitN!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'h , 's > SplitN < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8] , limit : usize ,) -> SplitN < 'h , 's > { let split = haystack . split_str (splitter) ; SplitN { split , limit , count : 0 } } }
    };
}

impl_95!()