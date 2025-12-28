macro_rules! deps {
    () => {
        Utf16UnreadHandle!();
        Utf16Source!();
        Unicode!();
        Utf16ReadHandle!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < 'a , 'b > Utf16ReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut Utf16Source < 'b >) -> Utf16ReadHandle < 'a , 'b > { Utf16ReadHandle { source : src } } # [inline (always)] pub fn read (self) -> (char , Utf16UnreadHandle < 'a , 'b >) { Utf16UnreadHandle :: new_char (self . source) } # [inline (always)] pub fn read_enum (self) -> (Unicode , Utf16UnreadHandle < 'a , 'b >) { Utf16UnreadHandle :: new_enum (self . source) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
    };
}

impl_335!()