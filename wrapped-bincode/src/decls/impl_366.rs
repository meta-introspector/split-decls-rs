macro_rules! deps {
    () => {
        BorrowDecoder!();
        BorrowDecode!();
        DecodeError!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < 'a , 'de : 'a , Context > BorrowDecode < 'de , Context > for & 'a str { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let slice = < & [u8] > :: borrow_decode (decoder) ? ; core :: str :: from_utf8 (slice) . map_err (| inner | DecodeError :: Utf8 { inner }) } }
    };
}

impl_366!();