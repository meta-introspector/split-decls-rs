macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecodeError!();
        BorrowDecode!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < 'de , Context , T : BorrowDecode < 'de , Context > > BorrowDecode < 'de , Context > for Wrapping < T > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Wrapping (T :: borrow_decode (decoder) ?)) } }
    };
}

impl_360!()