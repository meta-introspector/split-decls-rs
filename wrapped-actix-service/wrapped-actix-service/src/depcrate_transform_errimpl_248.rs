// Generated macro for impl_248 (impl)
macro_rules! Depcrate_transform_errimpl_248 {
() => {
// Module: crate::transform_err
// Provides: {"impl_248"}
// Dependencies: {}
impl < T , S , F , E , Req > Transform < S , Req > for TransformMapInitErr < T , S , Req , F , E > where T : Transform < S , Req > , F : Fn (T :: InitError) -> E + Clone , { type Response = T :: Response ; type Error = T :: Error ; type Transform = T :: Transform ; type InitError = E ; type Future = TransformMapInitErrFuture < T , S , F , E , Req > ; fn new_transform (& self , service : S) -> Self :: Future { TransformMapInitErrFuture { fut : self . transform . new_transform (service) , f : self . mapper . clone () , } } }
};
}
