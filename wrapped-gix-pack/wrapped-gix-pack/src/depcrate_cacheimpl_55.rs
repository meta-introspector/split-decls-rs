// Generated macro for impl_55 (impl)
macro_rules! Depcrate_cacheimpl_55 {
() => {
// Module: crate::cache
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : DecodeEntry + ? Sized > DecodeEntry for Box < T > { fn put (& mut self , pack_id : u32 , offset : u64 , data : & [u8] , kind : Kind , compressed_size : usize) { self . deref_mut () . put (pack_id , offset , data , kind , compressed_size) ; } fn get (& mut self , pack_id : u32 , offset : u64 , out : & mut Vec < u8 >) -> Option < (Kind , usize) > { self . deref_mut () . get (pack_id , offset , out) } }
};
}
