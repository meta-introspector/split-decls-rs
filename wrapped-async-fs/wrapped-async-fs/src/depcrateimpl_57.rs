// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl AsyncRead for File { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { if self . read_pos . is_none () { self . read_pos = Some (ready ! (self . as_mut () . poll_seek (cx , SeekFrom :: Current (0)))) ; } let n = ready ! (Pin :: new (self . unblock . get_mut ()) . poll_read (cx , buf)) ? ; if let Some (Ok (pos)) = self . read_pos . as_mut () { * pos += n as u64 ; } Poll :: Ready (Ok (n)) } }
};
}
