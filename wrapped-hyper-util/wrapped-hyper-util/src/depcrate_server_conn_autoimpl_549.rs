// Generated macro for impl_549 (impl)
macro_rules! Depcrate_server_conn_autoimpl_549 {
() => {
// Module: crate::server::conn::auto
// Provides: {"impl_549"}
// Dependencies: {}
impl < I > Future for ReadVersion < I > where I : Read + Unpin , { type Output = io :: Result < (Version , Rewind < I >) > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if * this . cancelled { return Poll :: Ready (Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "Cancelled"))) ; } let mut buf = ReadBuf :: uninit (& mut * this . buf) ; unsafe { buf . unfilled () . advance (* this . filled) ; } ; while buf . filled () . len () < H2_PREFACE . len () { let len = buf . filled () . len () ; ready ! (Pin :: new (this . io . as_mut () . unwrap ()) . poll_read (cx , buf . unfilled ())) ? ; * this . filled = buf . filled () . len () ; if buf . filled () . len () == len || buf . filled () [len ..] != H2_PREFACE [len .. buf . filled () . len ()] { * this . version = Version :: H1 ; break ; } } let io = this . io . take () . unwrap () ; let buf = buf . filled () . to_vec () ; Poll :: Ready (Ok ((* this . version , Rewind :: new_buffered (io , Bytes :: from (buf)) ,))) } }
};
}
