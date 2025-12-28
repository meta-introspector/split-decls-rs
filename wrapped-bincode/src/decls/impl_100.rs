macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecode!();
        BorrowDecoder!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Mutex < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Mutex :: new (t)) } }
    };
}

impl_100!();