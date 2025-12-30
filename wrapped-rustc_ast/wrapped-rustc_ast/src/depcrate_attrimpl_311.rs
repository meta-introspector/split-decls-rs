// Generated macro for impl_311 (impl)
macro_rules! Depcrate_attrimpl_311 {
() => {
// Module: crate::attr
// Provides: {"impl_311"}
// Dependencies: {}
impl AttrIdGenerator { pub fn new () -> Self { AttrIdGenerator (AtomicU32 :: new (0)) } pub fn mk_attr_id (& self) -> AttrId { let id = self . 0 . fetch_add (1 , Ordering :: Relaxed) ; assert ! (id != u32 :: MAX) ; AttrId :: from_u32 (id) } }
};
}
