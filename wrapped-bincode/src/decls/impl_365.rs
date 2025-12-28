macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecoder!();
        BorrowDecode!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < 'a , 'de : 'a , Context > BorrowDecode < 'de , Context > for & 'a [u8] { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = super :: decode_slice_len (decoder) ? ; decoder . claim_bytes_read (len) ? ; decoder . borrow_reader () . take_bytes (len) } }
    };
}

impl_365!();