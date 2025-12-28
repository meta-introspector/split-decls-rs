macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecodeError!();
        BorrowDecode!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Box < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Box :: new (t)) } }
    };
}

impl_61!();