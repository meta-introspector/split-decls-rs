macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecode!();
        BorrowDecoder!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Rc < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into ()) } }
    };
}

impl_75!()