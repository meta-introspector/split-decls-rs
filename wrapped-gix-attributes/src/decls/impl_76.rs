macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl < 'a > Lines < 'a > { # [doc = " Create a new instance to parse all attributes in all lines of the input `bytes`."] pub fn new (bytes : & 'a [u8]) -> Self { let bom = unicode_bom :: Bom :: from (bytes) ; Lines { lines : bytes [bom . len () ..] . lines () , line_no : 0 , } } }
    };
}

impl_76!();