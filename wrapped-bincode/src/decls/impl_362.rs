macro_rules! deps {
    () => {
        BorrowDecoder!();
        BorrowDecode!();
        DecodeError!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < 'de , Context , T : BorrowDecode < 'de , Context > > BorrowDecode < 'de , Context > for Reverse < T > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Reverse (T :: borrow_decode (decoder) ?)) } }
    };
}

impl_362!()