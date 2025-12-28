macro_rules! deps {
    () => {
        Utf16AstralHandle!();
        Utf16Destination!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < 'a , 'b > Utf16AstralHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut Utf16Destination < 'b >) -> Utf16AstralHandle < 'a , 'b > { Utf16AstralHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_ascii (self , ascii : u8) -> & 'a mut Utf16Destination < 'b > { self . dest . write_ascii (ascii) ; self . dest } # [inline (always)] pub fn write_bmp (self , bmp : u16) -> & 'a mut Utf16Destination < 'b > { self . dest . write_bmp (bmp) ; self . dest } # [inline (always)] pub fn write_bmp_excl_ascii (self , bmp : u16) -> & 'a mut Utf16Destination < 'b > { self . dest . write_bmp_excl_ascii (bmp) ; self . dest } # [inline (always)] pub fn write_upper_bmp (self , bmp : u16) -> & 'a mut Utf16Destination < 'b > { self . dest . write_upper_bmp (bmp) ; self . dest } # [inline (always)] pub fn write_astral (self , astral : u32) -> & 'a mut Utf16Destination < 'b > { self . dest . write_astral (astral) ; self . dest } # [inline (always)] pub fn write_surrogate_pair (self , high : u16 , low : u16) -> & 'a mut Utf16Destination < 'b > { self . dest . write_surrogate_pair (high , low) ; self . dest } # [inline (always)] pub fn write_big5_combination (self , combined : u16 , combining : u16 ,) -> & 'a mut Utf16Destination < 'b > { self . dest . write_big5_combination (combined , combining) ; self . dest } # [inline (always)] pub fn commit (self) -> & 'a mut Utf16Destination < 'b > { self . dest } }
    };
}

impl_323!()