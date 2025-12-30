// Generated macro for impl_294 (impl)
macro_rules! Depcrate_ioimpl_294 {
() => {
// Module: crate::io
// Provides: {"impl_294"}
// Dependencies: {}
impl < 'a , R > Future for FillBuf < 'a , R > where R : AsyncBufRead + Unpin + ? Sized , { type Output = Result < & 'a [u8] > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; let reader = this . reader . take () . expect ("polled `FillBuf` after completion") ; match Pin :: new (& mut * reader) . poll_fill_buf (cx) { Poll :: Ready (Ok (_)) => match Pin :: new (reader) . poll_fill_buf (cx) { Poll :: Ready (Ok (slice)) => Poll :: Ready (Ok (slice)) , poll => panic ! ("`poll_fill_buf()` was ready but now it isn't: {:?}" , poll) , } , Poll :: Ready (Err (err)) => Poll :: Ready (Err (err)) , Poll :: Pending => { this . reader = Some (reader) ; Poll :: Pending } } } }
};
}
