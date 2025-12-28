macro_rules! deps {
    () => {
        ByteThreeHandle!();
        ByteDestination!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'a , 'b > ByteThreeHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut ByteDestination < 'b >) -> ByteThreeHandle < 'a , 'b > { ByteThreeHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_one (self , first : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_one (first) ; self . dest } # [inline (always)] pub fn write_two (self , first : u8 , second : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_two (first , second) ; self . dest } # [inline (always)] pub fn write_three (self , first : u8 , second : u8 , third : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_three (first , second , third) ; self . dest } # [inline (always)] pub fn write_three_return_written (self , first : u8 , second : u8 , third : u8) -> usize { self . dest . write_three (first , second , third) ; self . dest . written () } }
    };
}

impl_349!();