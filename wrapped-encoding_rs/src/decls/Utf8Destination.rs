macro_rules! Utf8Destination {
    () => {
        pub struct Utf8Destination < 'a > { slice : & 'a mut [u8] , pos : usize , }
    };
}

Utf8Destination!();