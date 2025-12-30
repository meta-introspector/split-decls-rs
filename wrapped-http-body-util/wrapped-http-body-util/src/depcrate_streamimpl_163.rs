// Generated macro for impl_163 (impl)
macro_rules! Depcrate_streamimpl_163 {
() => {
// Module: crate::stream
// Provides: {"impl_163"}
// Dependencies: {}
impl < S , D , E > Body for StreamBody < S > where S : Stream < Item = Result < Frame < D > , E > > , D : Buf , { type Data = D ; type Error = E ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { match self . project () . stream . poll_next (cx) { Poll :: Ready (Some (result)) => Poll :: Ready (Some (result)) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
};
}
