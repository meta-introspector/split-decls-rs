// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl Body for String { type Data = Bytes ; type Error = Infallible ; fn poll_frame (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { if ! self . is_empty () { let s = std :: mem :: take (& mut * self) ; Poll :: Ready (Some (Ok (Frame :: data (s . into_bytes () . into ())))) } else { Poll :: Ready (None) } } fn is_end_stream (& self) -> bool { self . is_empty () } fn size_hint (& self) -> SizeHint { SizeHint :: with_exact (self . len () as u64) } }
};
}
