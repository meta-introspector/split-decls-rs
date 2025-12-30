// Generated macro for impl_151 (impl)
macro_rules! Depcrate_features_impl_stdimpl_151 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'de , T , S , Context > BorrowDecode < 'de , Context > for HashSet < T , S > where T : BorrowDecode < 'de , Context > + Eq + Hash , S : std :: hash :: BuildHasher + Default , { fn borrow_decode < D : BorrowDecoder < 'de , Context = Context > > (decoder : & mut D ,) -> Result < Self , DecodeError > { let len = crate :: de :: decode_slice_len (decoder) ? ; decoder . claim_container_read :: < T > (len) ? ; let mut map = HashSet :: with_capacity_and_hasher (len , S :: default ()) ; for _ in 0 .. len { decoder . unclaim_bytes_read (core :: mem :: size_of :: < T > ()) ; let key = T :: borrow_decode (decoder) ? ; map . insert (key) ; } Ok (map) } }
};
}
