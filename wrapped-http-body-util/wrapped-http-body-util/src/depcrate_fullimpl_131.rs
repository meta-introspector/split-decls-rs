// Generated macro for impl_131 (impl)
macro_rules! Depcrate_fullimpl_131 {
() => {
// Module: crate::full
// Provides: {"impl_131"}
// Dependencies: {}
impl < D > Body for Full < D > where D : Buf , { type Data = D ; type Error = Infallible ; fn poll_frame (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < D > , Self :: Error > > > { Poll :: Ready (self . data . take () . map (| d | Ok (Frame :: data (d)))) } fn is_end_stream (& self) -> bool { self . data . is_none () } fn size_hint (& self) -> SizeHint { self . data . as_ref () . map (| data | SizeHint :: with_exact (u64 :: try_from (data . remaining ()) . unwrap ())) . unwrap_or_else (| | SizeHint :: with_exact (0)) } }
};
}
