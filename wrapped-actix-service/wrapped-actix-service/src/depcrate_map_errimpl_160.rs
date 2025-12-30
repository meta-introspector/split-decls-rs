// Generated macro for impl_160 (impl)
macro_rules! Depcrate_map_errimpl_160 {
() => {
// Module: crate::map_err
// Provides: {"impl_160"}
// Dependencies: {}
impl < A , Req , F , E > Future for MapErrFuture < A , Req , F , E > where A : Service < Req > , F : Fn (A :: Error) -> E , { type Output = Result < A :: Response , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; this . fut . poll (cx) . map_err (this . f) } }
};
}
