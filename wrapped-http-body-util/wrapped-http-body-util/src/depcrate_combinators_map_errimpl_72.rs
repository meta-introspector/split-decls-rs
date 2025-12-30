// Generated macro for impl_72 (impl)
macro_rules! Depcrate_combinators_map_errimpl_72 {
() => {
// Module: crate::combinators::map_err
// Provides: {"impl_72"}
// Dependencies: {}
impl < B , F , E > Body for MapErr < B , F > where B : Body , F : FnMut (B :: Error) -> E , { type Data = B :: Data ; type Error = E ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match this . inner . poll_frame (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Ok (frame))) => Poll :: Ready (Some (Ok (frame))) , Poll :: Ready (Some (Err (err))) => Poll :: Ready (Some (Err ((this . f) (err)))) , } } fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } fn size_hint (& self) -> SizeHint { self . inner . size_hint () } }
};
}
