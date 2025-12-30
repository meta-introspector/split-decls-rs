// Generated macro for impl_250 (impl)
macro_rules! Depcrate_transform_errimpl_250 {
() => {
// Module: crate::transform_err
// Provides: {"impl_250"}
// Dependencies: {}
impl < T , S , F , E , Req > Future for TransformMapInitErrFuture < T , S , F , E , Req > where T : Transform < S , Req > , F : Fn (T :: InitError) -> E + Clone , { type Output = Result < T :: Transform , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (res) = this . fut . poll (cx) { Poll :: Ready (res . map_err (this . f)) } else { Poll :: Pending } } }
};
}
