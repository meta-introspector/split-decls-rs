macro_rules! BytesRef {
    () => {
        # [doc = " `BytesRef` is not a part of public API of bytes crate."] struct BytesRef < 'a > (& 'a [u8]) ;
    };
}

BytesRef!();