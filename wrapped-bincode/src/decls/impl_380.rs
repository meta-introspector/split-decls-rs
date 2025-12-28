macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for RefCell < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (RefCell :: new (t)) } }
    };
}

impl_380!();