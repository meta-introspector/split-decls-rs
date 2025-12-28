macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Range < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let min = T :: borrow_decode (decoder) ? ; let max = T :: borrow_decode (decoder) ? ; Ok (min .. max) } }
    };
}

impl_384!();