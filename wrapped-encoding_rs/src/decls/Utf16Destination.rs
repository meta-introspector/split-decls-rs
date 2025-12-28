macro_rules! Utf16Destination {
    () => {
        pub struct Utf16Destination < 'a > { slice : & 'a mut [u16] , pos : usize , }
    };
}

Utf16Destination!();