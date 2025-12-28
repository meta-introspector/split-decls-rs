macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > Range < 'a > { pub fn new (start_key : & 'a [u8] , end_key : & 'a [u8]) -> Range < 'a > { Range { start_key , end_key } } }
    };
}

impl_93!()