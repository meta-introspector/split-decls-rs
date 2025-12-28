macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for VecDeque < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: borrow_decode (decoder) ? . into ()) } }
    };
}

impl_50!();