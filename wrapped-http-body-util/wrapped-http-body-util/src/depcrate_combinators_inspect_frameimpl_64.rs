// Generated macro for impl_64 (impl)
macro_rules! Depcrate_combinators_inspect_frameimpl_64 {
() => {
// Module: crate::combinators::inspect_frame
// Provides: {"impl_64"}
// Dependencies: {}
impl < B , F > Body for InspectFrame < B , F > where B : Body , F : FnMut (& Frame < B :: Data >) , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match this . inner . poll_frame (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Err (err))) => Poll :: Ready (Some (Err (err))) , Poll :: Ready (Some (Ok (frame))) => { (this . f) (& frame) ; Poll :: Ready (Some (Ok (frame))) } } } fn size_hint (& self) -> http_body :: SizeHint { self . inner . size_hint () } fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } }
};
}
