macro_rules! deps {
    () => {
        BorrowDecode!();
        BorrowDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < 'de , T , Context > BorrowDecode < 'de , Context > for Arc < T > where T : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let t = T :: borrow_decode (decoder) ? ; Ok (Arc :: new (t)) } }
    };
}

impl_78!()