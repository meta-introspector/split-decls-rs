// Generated macro for impl_219 (impl)
macro_rules! Depcrate_cacheimpl_219 {
() => {
// Module: crate::cache
// Provides: {"impl_219"}
// Dependencies: {}
impl < S > From < S > for Cache < S > where S : gix_pack :: Find , { fn from (store : S) -> Self { Self { inner : store , pack_cache : None , new_pack_cache : None , object_cache : None , new_object_cache : None , } } }
};
}
