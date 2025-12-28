macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecode!();
        BorrowDecoder!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < 'de , Context > BorrowDecode < 'de , Context > for Arc < str > { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let decoded = String :: decode (decoder) ? ; Ok (decoded . into ()) } }
    };
}

impl_79!();