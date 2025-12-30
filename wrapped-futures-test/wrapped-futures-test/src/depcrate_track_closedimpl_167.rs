// Generated macro for impl_167 (impl)
macro_rules! Depcrate_track_closedimpl_167 {
() => {
// Module: crate::track_closed
// Provides: {"impl_167"}
// Dependencies: {}
impl < T : AsyncWrite > AsyncWrite for TrackClosed < T > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { if self . is_closed () { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , "Attempted to write after stream was closed" ,))) ; } self . project () . inner . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { if self . is_closed () { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , "Attempted to flush after stream was closed" ,))) ; } assert ! (! self . is_closed ()) ; self . project () . inner . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { if self . is_closed () { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , "Attempted to close after stream was closed" ,))) ; } let this = self . project () ; match this . inner . poll_close (cx) { Poll :: Ready (Ok (())) => { * this . closed = true ; Poll :: Ready (Ok (())) } other => other , } } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { if self . is_closed () { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Other , "Attempted to write after stream was closed" ,))) ; } self . project () . inner . poll_write_vectored (cx , bufs) } }
};
}
