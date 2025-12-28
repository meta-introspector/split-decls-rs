macro_rules! Utf8Source {
    () => {
        pub struct Utf8Source < 'a > { slice : & 'a [u8] , pos : usize , }
    };
}

Utf8Source!();