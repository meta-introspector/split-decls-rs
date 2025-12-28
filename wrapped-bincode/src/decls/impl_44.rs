macro_rules! deps {
    () => {
        BorrowDecoder!();
        DecodeError!();
        BorrowDecode!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'de , K , V , Context > BorrowDecode < 'de , Context > for BTreeMap < K , V > where K : BorrowDecode < 'de , Context > + Ord , V : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let mut map = BTreeMap :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let key = K :: borrow_decode (decoder) ? ; let value = V :: borrow_decode (decoder) ? ; map . insert (key , value) ; } Ok (map) } }
    };
}

impl_44!();