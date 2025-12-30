// Generated macro for impl_69 (impl)
macro_rules! Depcrate_cache_objectimpl_69 {
() => {
// Module: crate::cache::object
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : cache :: Object + ? Sized > cache :: Object for Box < T > { fn put (& mut self , id : gix_hash :: ObjectId , kind : gix_object :: Kind , data : & [u8]) { use std :: ops :: DerefMut ; self . deref_mut () . put (id , kind , data) ; } fn get (& mut self , id : & gix_hash :: ObjectId , out : & mut Vec < u8 >) -> Option < gix_object :: Kind > { use std :: ops :: DerefMut ; self . deref_mut () . get (id , out) } }
};
}
