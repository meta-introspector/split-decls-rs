// Generated macro for impl_135 (impl)
macro_rules! Depcrate_index_checkoutimpl_135 {
() => {
// Module: crate::index::checkout
// Provides: {"impl_135"}
// Dependencies: {}
impl gix :: objs :: Find for Empty { fn try_find < 'a > (& self , _id : & gix :: oid , buffer : & 'a mut Vec < u8 >) -> Result < Option < gix :: objs :: Data < 'a > > , Error > { buffer . clear () ; Ok (Some (gix :: objs :: Data { kind : gix :: object :: Kind :: Blob , data : buffer , })) } }
};
}
