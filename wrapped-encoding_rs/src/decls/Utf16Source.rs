macro_rules! Utf16Source {
    () => {
        pub struct Utf16Source < 'a > { slice : & 'a [u16] , pos : usize , }
    };
}

Utf16Source!()