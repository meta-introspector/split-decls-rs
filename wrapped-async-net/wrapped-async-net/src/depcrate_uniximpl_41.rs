// Generated macro for impl_41 (impl)
macro_rules! Depcrate_uniximpl_41 {
() => {
// Module: crate::unix
// Provides: {"impl_41"}
// Dependencies: {}
impl AsyncWrite for UnixStream { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { loop { match self . inner . get_ref () . write (buf) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => { self . writable = None ; return Poll :: Ready (res) ; } } if self . writable . is_none () { self . writable = Some (self . inner . clone () . writable_owned ()) ; } if let Some (f) = & mut self . writable { let res = ready ! (Pin :: new (f) . poll (cx)) ; self . writable = None ; res ? ; } } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { loop { match self . inner . get_ref () . flush () { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => { self . writable = None ; return Poll :: Ready (res) ; } } if self . writable . is_none () { self . writable = Some (self . inner . clone () . writable_owned ()) ; } if let Some (f) = & mut self . writable { let res = ready ! (Pin :: new (f) . poll (cx)) ; self . writable = None ; res ? ; } } } fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (self . inner . get_ref () . shutdown (Shutdown :: Write)) } }
};
}
