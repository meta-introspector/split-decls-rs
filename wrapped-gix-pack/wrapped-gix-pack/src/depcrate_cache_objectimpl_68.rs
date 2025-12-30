// Generated macro for impl_68 (impl)
macro_rules! Depcrate_cache_objectimpl_68 {
() => {
// Module: crate::cache::object
// Provides: {"impl_68"}
// Dependencies: {}
impl cache :: Object for Never { # [doc = " Noop"] fn put (& mut self , _id : gix_hash :: ObjectId , _kind : gix_object :: Kind , _data : & [u8]) { } # [doc = " Noop"] fn get (& mut self , _id : & gix_hash :: ObjectId , _out : & mut Vec < u8 >) -> Option < gix_object :: Kind > { None } }
};
}
