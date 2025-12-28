macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecoder!();
        BorrowDecode!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'de , Context > BorrowDecode < 'de , Context > for & 'de Path { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let str = < & 'de str > :: borrow_decode (decoder) ? ; Ok (Path :: new (str)) } }
    };
}

impl_108!()