// Generated macro for impl_2154 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2154 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2154"}
// Dependencies: {}
impl < R > Future for SeekRelative < '_ , R > where R : AsyncRead + AsyncSeek , { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let offset = self . offset ; if self . first { self . first = false ; self . inner . as_mut () . poll_seek_relative (cx , offset) } else { self . inner . as_mut () . as_mut () . poll_seek (cx , SeekFrom :: Current (offset)) . map (| res | res . map (| _ | ())) } } }
};
}
