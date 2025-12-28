macro_rules! deps {
    () => {
        Utf8Destination!();
        Utf8BmpHandle!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'a , 'b > Utf8BmpHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut Utf8Destination < 'b >) -> Utf8BmpHandle < 'a , 'b > { Utf8BmpHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_ascii (self , ascii : u8) -> & 'a mut Utf8Destination < 'b > { self . dest . write_ascii (ascii) ; self . dest } # [inline (always)] pub fn write_bmp (self , bmp : u16) -> & 'a mut Utf8Destination < 'b > { self . dest . write_bmp (bmp) ; self . dest } # [inline (always)] pub fn write_bmp_excl_ascii (self , bmp : u16) -> & 'a mut Utf8Destination < 'b > { self . dest . write_bmp_excl_ascii (bmp) ; self . dest } # [inline (always)] pub fn write_mid_bmp (self , bmp : u16) -> & 'a mut Utf8Destination < 'b > { self . dest . write_mid_bmp (bmp) ; self . dest } # [inline (always)] pub fn write_upper_bmp (self , bmp : u16) -> & 'a mut Utf8Destination < 'b > { self . dest . write_upper_bmp (bmp) ; self . dest } # [inline (always)] pub fn commit (self) -> & 'a mut Utf8Destination < 'b > { self . dest } }
    };
}

impl_327!()