macro_rules! deps {
    () => {
        BorrowDecode!();
        DecodeError!();
        BorrowDecoder!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for Box < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into_boxed_slice ()) } }
    };
}

impl_64!();