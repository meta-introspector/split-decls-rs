// Generated macro for impl_3196 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3196 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3196"}
// Dependencies: {}
impl BufRead for CopyToBuffer < '_ > { # [allow (unsafe_code)] fn fill_buf (& mut self) -> std :: io :: Result < & [u8] > { if self . data_slice () . is_empty () { unsafe { if ! self . ptr . is_null () { pq_sys :: PQfreemem (self . ptr as * mut ffi :: c_void) ; self . ptr = std :: ptr :: null_mut () ; } let len = pq_sys :: PQgetCopyData (self . conn . internal_connection . as_ptr () , & mut self . ptr , 0) ; match len { len if len >= 0 => { self . len = 1 + usize :: try_from (len) . map_err (| e | std :: io :: Error :: new (std :: io :: ErrorKind :: Other , e)) ? } - 1 => self . len = 0 , _ => { let error = self . conn . last_error_message () ; return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , error)) ; } } self . offset = 0 ; } } Ok (self . data_slice ()) } fn consume (& mut self , amt : usize) { self . offset = usize :: min (self . len , self . offset + amt) ; } }
};
}
