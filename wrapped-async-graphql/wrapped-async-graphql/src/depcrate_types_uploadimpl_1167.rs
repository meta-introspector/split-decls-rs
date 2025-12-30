// Generated macro for impl_1167 (impl)
macro_rules! Depcrate_types_uploadimpl_1167 {
() => {
// Module: crate::types::upload
// Provides: {"impl_1167"}
// Dependencies: {}
impl Upload { # [doc = " Get the upload value."] pub fn value (& self , ctx : & Context < '_ >) -> std :: io :: Result < UploadValue > { ctx . query_env . uploads [self . 0] . try_clone () } }
};
}
