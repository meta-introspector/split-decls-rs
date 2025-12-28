macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
        Decode!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < Context , K , V , S > Decode < Context > for HashMap < K , V , S > where K : Decode < Context > + Eq + std :: hash :: Hash , V : Decode < Context > , S : std :: hash :: BuildHasher + Default , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let hash_builder : S = Default :: default () ; let mut map = HashMap :: with_capacity_and_hasher (len , hash_builder) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let k = K :: decode (decoder) ? ; let v = V :: decode (decoder) ? ; map . insert (k , v) ; } Ok (map) } }
    };
}

impl_131!();