// Generated macro for impl_220 (impl)
macro_rules! Depcrate_cacheimpl_220 {
() => {
// Module: crate::cache
// Provides: {"impl_220"}
// Dependencies: {}
impl < S : Clone > Clone for Cache < S > { fn clone (& self) -> Self { Cache { inner : self . inner . clone () , new_pack_cache : self . new_pack_cache . clone () , new_object_cache : self . new_object_cache . clone () , pack_cache : self . new_pack_cache . as_ref () . map (| create | RefCell :: new (create ())) , object_cache : self . new_object_cache . as_ref () . map (| create | RefCell :: new (create ())) , } } }
};
}
