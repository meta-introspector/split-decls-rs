// Generated macro for impl_easy_getters (macro)
macro_rules! Depcrate_multiimpl_easy_getters {
() => {
// Module: crate::multi
// Provides: {"impl_easy_getters"}
// Dependencies: {}
macro_rules ! impl_easy_getters { () => { impl_easy_getters ! { time_condition_unmet -> bool , effective_url -> Option <& str >, effective_url_bytes -> Option <& [u8] >, response_code -> u32 , http_connectcode -> u32 , filetime -> Option < i64 >, download_size -> f64 , content_length_download -> f64 , total_time -> Duration , namelookup_time -> Duration , connect_time -> Duration , appconnect_time -> Duration , pretransfer_time -> Duration , starttransfer_time -> Duration , redirect_time -> Duration , redirect_count -> u32 , redirect_url -> Option <& str >, redirect_url_bytes -> Option <& [u8] >, header_size -> u64 , request_size -> u64 , content_type -> Option <& str >, content_type_bytes -> Option <& [u8] >, os_errno -> i32 , primary_ip -> Option <& str >, primary_port -> u16 , local_ip -> Option <& str >, local_port -> u16 , cookies -> List , } } ; ($ ($ name : ident -> $ ret : ty ,) *) => { $ (impl_easy_getters ! ($ name , $ ret , concat ! ("Same as [`Easy2::" , stringify ! ($ name) , "`](../easy/struct.Easy2.html#method." , stringify ! ($ name) , ").")) ;) * } ; ($ name : ident , $ ret : ty , $ doc : expr) => { # [doc = $ doc] pub fn $ name (& mut self) -> Result <$ ret , Error > { self . easy .$ name () } } ; }
};
}
