macro_rules! deps {
    () => {
        Find!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'h , 'n > Find < 'h , 'n > { fn new (haystack : & 'h [u8] , needle : & 'n [u8]) -> Find < 'h , 'n > { Find { it : memmem :: find_iter (haystack , needle) , haystack , needle } } }
    };
}

impl_71!()