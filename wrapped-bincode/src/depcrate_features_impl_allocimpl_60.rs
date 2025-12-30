// Generated macro for impl_60 (impl)
macro_rules! Depcrate_features_impl_allocimpl_60 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'de , T , Context > BorrowDecode < 'de , Context > for BTreeSet < T > where T : BorrowDecode < 'de , Context > + Ord , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < T > (len) ? ; let mut map = BTreeSet :: new () ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; let key = T :: borrow_decode (decoder) ? ; map . insert (key) ; } Ok (map) } }
};
}
