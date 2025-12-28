macro_rules! deps {
    () => {
        Decode!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < Context , T , S > Decode < Context > for HashSet < T , S > where T : Decode < Context > + Eq + Hash , S : std :: hash :: BuildHasher + Default , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < T > (len) ? ; let hash_builder : S = Default :: default () ; let mut map : HashSet < T , S > = HashSet :: with_capacity_and_hasher (len , hash_builder) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; let key = T :: decode (decoder) ? ; map . insert (key) ; } Ok (map) } }
    };
}

impl_133!()