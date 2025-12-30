// Generated macro for impl_2147 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2147 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2147"}
// Dependencies: {}
impl < R : AsyncRead + AsyncSeek > BufReader < R > { # [doc = " Seeks relative to the current position. If the new position lies within the buffer,"] # [doc = " the buffer will not be flushed, allowing for more efficient seeks."] # [doc = " This method does not return the location of the underlying reader, so the caller"] # [doc = " must track this information themselves if it is required."] pub fn seek_relative (self : Pin < & mut Self > , offset : i64) -> SeekRelative < '_ , R > { SeekRelative { inner : self , offset , first : true } } # [doc = " Attempts to seek relative to the current position. If the new position lies within the buffer,"] # [doc = " the buffer will not be flushed, allowing for more efficient seeks."] # [doc = " This method does not return the location of the underlying reader, so the caller"] # [doc = " must track this information themselves if it is required."] pub fn poll_seek_relative (self : Pin < & mut Self > , cx : & mut Context < '_ > , offset : i64 ,) -> Poll < io :: Result < () > > { let pos = self . pos as u64 ; if offset < 0 { if let Some (new_pos) = pos . checked_sub ((- offset) as u64) { * self . project () . pos = new_pos as usize ; return Poll :: Ready (Ok (())) ; } } else if let Some (new_pos) = pos . checked_add (offset as u64) { if new_pos <= self . cap as u64 { * self . project () . pos = new_pos as usize ; return Poll :: Ready (Ok (())) ; } } self . poll_seek (cx , SeekFrom :: Current (offset)) . map (| res | res . map (| _ | ())) } }
};
}
