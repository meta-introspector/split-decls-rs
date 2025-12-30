// Generated macro for impl_208 (impl)
macro_rules! Depcrate_pack_verifyimpl_208 {
() => {
// Module: crate::pack::verify
// Provides: {"impl_208"}
// Dependencies: {}
impl < const SIZE : usize > pack :: cache :: DecodeEntry for EitherCache < SIZE > { fn put (& mut self , pack_id : u32 , offset : u64 , data : & [u8] , kind : object :: Kind , compressed_size : usize) { match self { EitherCache :: Left (v) => v . put (pack_id , offset , data , kind , compressed_size) , EitherCache :: Right (v) => v . put (pack_id , offset , data , kind , compressed_size) , } } fn get (& mut self , pack_id : u32 , offset : u64 , out : & mut Vec < u8 >) -> Option < (object :: Kind , usize) > { match self { EitherCache :: Left (v) => v . get (pack_id , offset , out) , EitherCache :: Right (v) => v . get (pack_id , offset , out) , } } }
};
}
