macro_rules! deps {
    () => {
        BorrowReader!();
        Reader!();
    };
}

macro_rules! SliceReader {
    () => {
        deps!();
        # [doc = " A reader type for `&[u8]` slices. Implements both [Reader] and [BorrowReader], and thus can be used for borrowed data."] pub struct SliceReader < 'storage > { pub (crate) slice : & 'storage [u8] , }
    };
}

SliceReader!();