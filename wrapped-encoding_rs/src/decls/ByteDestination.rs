macro_rules! ByteDestination {
    () => {
        pub struct ByteDestination < 'a > { slice : & 'a mut [u8] , # [doc = " Pointer to the original start of the slice. It's never dereferenced."] start : * const u8 , }
    };
}

ByteDestination!();