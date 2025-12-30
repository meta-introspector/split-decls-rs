// Generated macro for impl_81 (impl)
macro_rules! Depcrate_combinators_map_frameimpl_81 {
() => {
// Module: crate::combinators::map_frame
// Provides: {"impl_81"}
// Dependencies: {}
impl < B , F , B2 > Body for MapFrame < B , F > where B : Body , F : FnMut (Frame < B :: Data >) -> Frame < B2 > , B2 : Buf , { type Data = B2 ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match this . inner . poll_frame (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Ok (frame))) => Poll :: Ready (Some (Ok ((this . f) (frame)))) , Poll :: Ready (Some (Err (err))) => Poll :: Ready (Some (Err (err))) , } } fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } }
};
}
