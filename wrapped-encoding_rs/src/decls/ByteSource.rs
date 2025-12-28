macro_rules! ByteSource {
    () => {
        pub struct ByteSource < 'a > { slice : & 'a [u8] , pos : usize , }
    };
}

ByteSource!()