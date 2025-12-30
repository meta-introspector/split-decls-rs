// Generated macro for impl_106 (impl)
macro_rules! Depcrate_eitherimpl_106 {
() => {
// Module: crate::either
// Provides: {"impl_106"}
// Dependencies: {}
impl < L , R , Data > Body for Either < L , R > where L : Body < Data = Data > , R : Body < Data = Data > , L :: Error : Into < Box < dyn Error + Send + Sync > > , R :: Error : Into < Box < dyn Error + Send + Sync > > , Data : Buf , { type Data = Data ; type Error = Box < dyn Error + Send + Sync > ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { match self . project () { EitherProj :: Left (left) => left . poll_frame (cx) . map (| poll | poll . map (| opt | opt . map_err (Into :: into))) , EitherProj :: Right (right) => right . poll_frame (cx) . map (| poll | poll . map (| opt | opt . map_err (Into :: into))) , } } fn is_end_stream (& self) -> bool { match self { Either :: Left (left) => left . is_end_stream () , Either :: Right (right) => right . is_end_stream () , } } fn size_hint (& self) -> SizeHint { match self { Either :: Left (left) => left . size_hint () , Either :: Right (right) => right . size_hint () , } } }
};
}
