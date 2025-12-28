macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecoder!();
        BorrowDecode!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'de , Context > BorrowDecode < 'de , Context > for Rc < str > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
    };
}

impl_72!()