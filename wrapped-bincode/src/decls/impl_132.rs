macro_rules! deps {
    () => {
        DecodeError!();
        BorrowDecoder!();
        BorrowDecode!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'de , K , V , S , Context > BorrowDecode < 'de , Context > for HashMap < K , V , S > where K : BorrowDecode < 'de , Context > + Eq + std :: hash :: Hash , V : BorrowDecode < 'de , Context > , S : std :: hash :: BuildHasher + Default , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let hash_builder : S = Default :: default () ; let mut map = HashMap :: with_capacity_and_hasher (len , hash_builder) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let k = K :: borrow_decode (decoder) ? ; let v = V :: borrow_decode (decoder) ? ; map . insert (k , v) ; } Ok (map) } }
    };
}

impl_132!();