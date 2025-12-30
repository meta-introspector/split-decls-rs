// Generated macro for impl_126 (impl)
macro_rules! Depcrate_recv_streamimpl_126 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_126"}
// Dependencies: {}
impl Future for ReadToEnd < '_ > { type Output = Result < Vec < u8 > , ReadToEndError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_read_chunk (cx , usize :: MAX , false)) ? { Some (chunk) => { self . start = self . start . min (chunk . offset) ; let end = chunk . bytes . len () as u64 + chunk . offset ; if (end - self . start) > self . size_limit as u64 { return Poll :: Ready (Err (ReadToEndError :: TooLong)) ; } self . end = self . end . max (end) ; self . read . push ((chunk . bytes , chunk . offset)) ; } None => { if self . end == 0 { return Poll :: Ready (Ok (Vec :: new ())) ; } let start = self . start ; let mut buffer = vec ! [0 ; (self . end - start) as usize] ; for (data , offset) in self . read . drain (..) { let offset = (offset - start) as usize ; buffer [offset .. offset + data . len ()] . copy_from_slice (& data) ; } return Poll :: Ready (Ok (buffer)) ; } } } } }
};
}
