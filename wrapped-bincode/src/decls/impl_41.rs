macro_rules! deps {
    () => {
        BorrowDecoder!();
        BorrowDecode!();
        DecodeError!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'de , T , Context > BorrowDecode < 'de , Context > for BinaryHeap < T > where T : BorrowDecode < 'de , Context > + Ord , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { Ok (Vec :: < T > :: borrow_decode (decoder) ? . into ()) } }
    };
}

impl_41!();