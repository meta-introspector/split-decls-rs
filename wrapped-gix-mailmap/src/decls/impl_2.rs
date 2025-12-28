macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'a > Lines < 'a > { pub (crate) fn new (input : & 'a [u8]) -> Self { Lines { lines : input . as_bstr () . lines () , line_no : 0 , } } }
    };
}

impl_2!();