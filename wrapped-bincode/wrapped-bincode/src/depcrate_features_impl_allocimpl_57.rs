// Generated macro for impl_57 (impl)
macro_rules! Depcrate_features_impl_allocimpl_57 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'de , K , V , Context > BorrowDecode < 'de , Context > for BTreeMap < K , V > where K : BorrowDecode < 'de , Context > + Ord , V : BorrowDecode < 'de , Context > , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < (K , V) > (len) ? ; let mut map = BTreeMap :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < (K , V) > ()) ; let key = K :: borrow_decode (decoder) ? ; let value = V :: borrow_decode (decoder) ? ; map . insert (key , value) ; } Ok (map) } }
};
}
