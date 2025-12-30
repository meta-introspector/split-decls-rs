// Generated macro for impl_864 (impl)
macro_rules! Depcrate_serverimpl_864 {
() => {
// Module: crate::server
// Provides: {"impl_864"}
// Dependencies: {}
impl < T , B > Future for ReadPreface < T , B > where T : AsyncRead + Unpin , B : Buf , { type Output = Result < Codec < T , B > , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut buf = [0 ; 24] ; let mut rem = PREFACE . len () - self . pos ; while rem > 0 { let mut buf = ReadBuf :: new (& mut buf [.. rem]) ; ready ! (Pin :: new (self . inner_mut ()) . poll_read (cx , & mut buf)) . map_err (crate :: Error :: from_io) ? ; let n = buf . filled () . len () ; if n == 0 { return Poll :: Ready (Err (crate :: Error :: from_io (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "connection closed before reading preface" ,)))) ; } if & PREFACE [self . pos .. self . pos + n] != buf . filled () { proto_err ! (conn : "read_preface: invalid preface") ; return Poll :: Ready (Err (Error :: library_go_away (Reason :: PROTOCOL_ERROR) . into ())) ; } self . pos += n ; rem -= n ; } Poll :: Ready (Ok (self . codec . take () . unwrap ())) } }
};
}
