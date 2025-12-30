// Generated macro for impl_48 (impl)
macro_rules! Depcrate_combinators_fuseimpl_48 {
() => {
// Module: crate::combinators::fuse
// Provides: {"impl_48"}
// Dependencies: {}
impl < B > Body for Fuse < B > where B : Body + Unpin , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < B :: Data > , B :: Error > > > { let Self { inner } = self . get_mut () ; let poll = inner . as_mut () . map (| mut inner | match Pin :: new (& mut inner) . poll_frame (cx) { frame @ Poll :: Ready (Some (Ok (_))) => (frame , inner . is_end_stream ()) , end @ Poll :: Ready (Some (Err (_)) | None) => (end , true) , poll @ Poll :: Pending => (poll , false) , }) ; if let Some ((frame , eos)) = poll { eos . then (| | inner . take ()) ; frame } else { Poll :: Ready (None) } } fn is_end_stream (& self) -> bool { self . inner . is_none () } fn size_hint (& self) -> SizeHint { self . inner . as_ref () . map (B :: size_hint) . unwrap_or_else (| | SizeHint :: with_exact (0)) } }
};
}
