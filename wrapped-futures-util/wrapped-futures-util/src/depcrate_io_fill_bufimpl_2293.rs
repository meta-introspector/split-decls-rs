// Generated macro for impl_2293 (impl)
macro_rules! Depcrate_io_fill_bufimpl_2293 {
() => {
// Module: crate::io::fill_buf
// Provides: {"impl_2293"}
// Dependencies: {}
impl < 'a , R > Future for FillBuf < 'a , R > where R : AsyncBufRead + ? Sized + Unpin , { type Output = io :: Result < & 'a [u8] > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; let reader = this . reader . take () . expect ("Polled FillBuf after completion") ; match Pin :: new (& mut * reader) . poll_fill_buf (cx) { Poll :: Ready (Ok (slice)) => { let slice : & 'a [u8] = unsafe { slice :: from_raw_parts (slice . as_ptr () , slice . len ()) } ; Poll :: Ready (Ok (slice)) } Poll :: Ready (Err (err)) => Poll :: Ready (Err (err)) , Poll :: Pending => { this . reader = Some (reader) ; Poll :: Pending } } } }
};
}
