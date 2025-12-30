// Generated macro for impl_472 (impl)
macro_rules! Depcrate_ffi_ioimpl_472 {
() => {
// Module: crate::ffi::io
// Provides: {"impl_472"}
// Dependencies: {}
impl Write for hyper_io { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < std :: io :: Result < usize > > { let buf_ptr = buf . as_ptr () ; let buf_len = buf . len () ; match (self . write) (self . userdata , hyper_context :: wrap (cx) , buf_ptr , buf_len) { HYPER_IO_PENDING => Poll :: Pending , HYPER_IO_ERROR => Poll :: Ready (Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "io error" ,))) , ok => Poll :: Ready (Ok (ok)) , } } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < std :: io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < std :: io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
