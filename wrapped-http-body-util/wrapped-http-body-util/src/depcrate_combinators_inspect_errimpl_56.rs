// Generated macro for impl_56 (impl)
macro_rules! Depcrate_combinators_inspect_errimpl_56 {
() => {
// Module: crate::combinators::inspect_err
// Provides: {"impl_56"}
// Dependencies: {}
impl < B , F > Body for InspectErr < B , F > where B : Body , F : FnMut (& B :: Error) , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match this . inner . poll_frame (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Ok (frame))) => Poll :: Ready (Some (Ok (frame))) , Poll :: Ready (Some (Err (err))) => { (this . f) (& err) ; Poll :: Ready (Some (Err (err))) } } } fn size_hint (& self) -> http_body :: SizeHint { self . inner . size_hint () } fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } }
};
}
