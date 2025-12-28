macro_rules! deps {
    () => {
        ByteDestination!();
        ByteOneHandle!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'a , 'b > ByteOneHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut ByteDestination < 'b >) -> ByteOneHandle < 'a , 'b > { ByteOneHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_one (self , first : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_one (first) ; self . dest } }
    };
}

impl_345!();