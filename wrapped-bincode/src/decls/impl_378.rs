macro_rules! deps {
    () => {
        BorrowDecode!();
        DecodeError!();
        BorrowDecoder!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Cell < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Cell :: new (t)) } }
    };
}

impl_378!();