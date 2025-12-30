// Generated macro for impl_150 (impl)
macro_rules! Depcrate_features_impl_stdimpl_150 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_150"}
// Dependencies: {}
impl < Context , T , S > Decode < Context > for HashSet < T , S > where T : Decode < Context > + Eq + Hash , S : std :: hash :: BuildHasher + Default , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < T > (len) ? ; let hash_builder : S = Default :: default () ; let mut map : HashSet < T , S > = HashSet :: with_capacity_and_hasher (len , hash_builder) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; let key = T :: decode (decoder) ? ; map . insert (key) ; } Ok (map) } }
};
}
