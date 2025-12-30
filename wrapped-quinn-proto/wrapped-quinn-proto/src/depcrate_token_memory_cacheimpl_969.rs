// Generated macro for impl_969 (impl)
macro_rules! Depcrate_token_memory_cacheimpl_969 {
() => {
// Module: crate::token_memory_cache
// Provides: {"impl_969"}
// Dependencies: {}
impl TokenStore for TokenMemoryCache { fn insert (& self , server_name : & str , token : Bytes) { trace ! (% server_name , "storing token") ; self . 0 . lock () . unwrap () . store (server_name , token) } fn take (& self , server_name : & str) -> Option < Bytes > { let token = self . 0 . lock () . unwrap () . take (server_name) ; trace ! (% server_name , found =% token . is_some () , "taking token") ; token } }
};
}
