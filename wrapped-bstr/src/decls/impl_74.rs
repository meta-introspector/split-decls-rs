macro_rules! deps {
    () => {
        FindReverse!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'h , 'n > FindReverse < 'h , 'n > { fn new (haystack : & 'h [u8] , needle : & 'n [u8]) -> FindReverse < 'h , 'n > { FindReverse { it : memmem :: rfind_iter (haystack , needle) , haystack , needle , } } fn haystack (& self) -> & 'h [u8] { self . haystack } fn needle (& self) -> & 'n [u8] { self . needle } }
    };
}

impl_74!()