// Generated macro for impl_2386 (impl)
macro_rules! Depcrate_io_read_lineimpl_2386 {
() => {
// Module: crate::io::read_line
// Provides: {"impl_2386"}
// Dependencies: {}
impl < R : AsyncBufRead + ? Sized + Unpin > Future for ReadLine < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , read , finished : _ } = & mut * self ; let ret = ready ! (read_line_internal (Pin :: new (reader) , cx , buf , bytes , read)) ; self . finished = true ; Poll :: Ready (ret) } }
};
}
