// Generated macro for impl_56 (impl)
macro_rules! Depcrate_features_impl_allocimpl_56 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_56"}
// Dependencies: {}
impl < Context , K , V > Decode < Context > for BTreeMap < K , V > where K : Decode < Context > + Ord , V : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let mut map = BTreeMap :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let key = K :: decode (decoder) ? ; let value = V :: decode (decoder) ? ; map . insert (key , value) ; } Ok (map) } }
};
}
