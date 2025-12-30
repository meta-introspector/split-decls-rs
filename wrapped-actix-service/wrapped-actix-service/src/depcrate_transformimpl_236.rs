// Generated macro for impl_236 (impl)
macro_rules! Depcrate_transformimpl_236 {
() => {
// Module: crate::transform
// Provides: {"impl_236"}
// Dependencies: {}
impl < T , S , Req > ServiceFactory < Req > for ApplyTransform < T , S , Req > where S : ServiceFactory < Req > , T : Transform < S :: Service , Req , InitError = S :: InitError > , { type Response = T :: Response ; type Error = T :: Error ; type Config = S :: Config ; type Service = T :: Transform ; type InitError = T :: InitError ; type Future = ApplyTransformFuture < T , S , Req > ; fn new_service (& self , cfg : S :: Config) -> Self :: Future { ApplyTransformFuture { store : self . 0 . clone () , state : ApplyTransformFutureState :: A { fut : self . 0 . as_ref () . 1 . new_service (cfg) , } , } } }
};
}
