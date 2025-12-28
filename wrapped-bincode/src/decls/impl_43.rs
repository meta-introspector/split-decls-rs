macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < Context , K , V > Decode < Context > for BTreeMap < K , V > where K : Decode < Context > + Ord , V : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let mut map = BTreeMap :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let key = K :: decode (decoder) ? ; let value = V :: decode (decoder) ? ; map . insert (key , value) ; } Ok (map) } }
    };
}

impl_43!()