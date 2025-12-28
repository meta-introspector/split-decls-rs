macro_rules! deps {
    () => {
        Utf8Source!();
        Utf8UnreadHandle!();
        Unicode!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'a , 'b > Utf8UnreadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new_char (source : & 'a mut Utf8Source < 'b >) -> (char , Self) { let old_pos = source . pos ; let character = source . read () ; (character , Self { source , old_pos }) } # [inline (always)] fn new_enum (source : & 'a mut Utf8Source < 'b >) -> (Unicode , Self) { let old_pos = source . pos ; let character = source . read_enum () ; (character , Self { source , old_pos }) } # [inline (always)] pub fn unread (self) -> usize { self . source . pos = self . old_pos ; self . old_pos } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } # [inline (always)] pub fn commit (self) -> & 'a mut Utf8Source < 'b > { self . source } }
    };
}

impl_343!()