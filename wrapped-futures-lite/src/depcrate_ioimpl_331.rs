// Generated macro for impl_331 (impl)
macro_rules! Depcrate_ioimpl_331 {
() => {
// Module: crate::io
// Provides: {"impl_331"}
// Dependencies: {}
impl < R : AsyncRead + Unpin > Stream for Bytes < R > { type Item = Result < u8 > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut byte = 0 ; let rd = Pin :: new (& mut self . inner) ; match ready ! (rd . poll_read (cx , std :: slice :: from_mut (& mut byte))) { Ok (0) => Poll :: Ready (None) , Ok (..) => Poll :: Ready (Some (Ok (byte))) , Err (ref e) if e . kind () == ErrorKind :: Interrupted => Poll :: Pending , Err (e) => Poll :: Ready (Some (Err (e))) , } } }
};
}
