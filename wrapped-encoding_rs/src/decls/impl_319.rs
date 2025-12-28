macro_rules! deps {
    () => {
        ByteSource!();
        ByteUnreadHandle!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < 'a , 'b > ByteUnreadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (src : & 'a mut ByteSource < 'b >) -> ByteUnreadHandle < 'a , 'b > { ByteUnreadHandle { source : src } } # [inline (always)] pub fn unread (self) -> usize { self . source . unread () } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } # [inline (always)] pub fn commit (self) -> & 'a mut ByteSource < 'b > { self . source } }
    };
}

impl_319!()