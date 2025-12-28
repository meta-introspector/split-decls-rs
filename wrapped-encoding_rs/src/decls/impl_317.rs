macro_rules! deps {
    () => {
        ByteUnreadHandle!();
        ByteReadHandle!();
        ByteSource!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < 'a , 'b > ByteReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut ByteSource < 'b >) -> ByteReadHandle < 'a , 'b > { ByteReadHandle { source : src } } # [inline (always)] pub fn read (self) -> (u8 , ByteUnreadHandle < 'a , 'b >) { let byte = self . source . read () ; let handle = ByteUnreadHandle :: new (self . source) ; (byte , handle) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
    };
}

impl_317!();