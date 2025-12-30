// Generated macro for impl_59 (impl)
macro_rules! Depcrate_features_impl_allocimpl_59 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_59"}
// Dependencies: {}
impl < Context , T > Decode < Context > for BTreeSet < T > where T : Decode < Context > + Ord , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < T > (len) ? ; let mut map = BTreeSet :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; let key = T :: decode (decoder) ? ; map . insert (key) ; } Ok (map) } }
};
}
