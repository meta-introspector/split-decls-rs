macro_rules! deps {
    () => {
        Unicode!();
        Utf8UnreadHandle!();
        Utf8ReadHandle!();
        Utf8Source!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < 'a , 'b > Utf8ReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (source : & 'a mut Utf8Source < 'b >) -> Utf8ReadHandle < 'a , 'b > { Utf8ReadHandle { source } } # [inline (always)] pub fn read (self) -> (char , Utf8UnreadHandle < 'a , 'b >) { Utf8UnreadHandle :: new_char (self . source) } # [inline (always)] pub fn read_enum (self) -> (Unicode , Utf8UnreadHandle < 'a , 'b >) { Utf8UnreadHandle :: new_enum (self . source) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
    };
}

impl_341!();