macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'cow , T , Context > BorrowDecode < 'cow , Context > for Cow < 'cow , T > where T : ToOwned + ? Sized , & 'cow T : BorrowDecode < 'cow , Context > , { fn borrow_decode < D : BorrowDecoder < 'cow , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = < & T > :: borrow_decode (decoder) ? ; Ok (Cow :: Borrowed (t)) } }
    };
}

impl_66!();