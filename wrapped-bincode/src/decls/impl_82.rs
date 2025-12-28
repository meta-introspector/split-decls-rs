macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecodeError!();
        BorrowDecode!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl < 'de , T , Context > BorrowDecode < 'de , Context > for Arc < [T] > where T : BorrowDecode < 'de , Context > + 'de , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let vec = Vec :: borrow_decode (decoder) ? ; Ok (vec . into ()) } }
    };
}

impl_82!();