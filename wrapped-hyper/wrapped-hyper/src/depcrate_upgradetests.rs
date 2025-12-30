// Generated macro for tests (module)
macro_rules! Depcrate_upgradetests {
() => {
// Module: crate::upgrade
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] # [cfg (test)] mod tests { use super :: * ; # [test] fn upgraded_downcast () { let upgraded = Upgraded :: new (Mock , Bytes :: new ()) ; let upgraded = upgraded . downcast :: < crate :: common :: io :: Compat < std :: io :: Cursor < Vec < u8 > > > > () . unwrap_err () ; upgraded . downcast :: < Mock > () . unwrap () ; } struct Mock ; impl Read for Mock { fn poll_read (self : Pin < & mut Self > , _cx : & mut Context < '_ > , _buf : ReadBufCursor < '_ > ,) -> Poll < io :: Result < () > > { unreachable ! ("Mock::poll_read") } } impl Write for Mock { fn poll_write (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (buf . len ())) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { unreachable ! ("Mock::poll_flush") } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { unreachable ! ("Mock::poll_shutdown") } } }
};
}
