// Generated macro for impl_179 (impl)
macro_rules! Depcrate_map_init_errimpl_179 {
() => {
// Module: crate::map_init_err
// Provides: {"impl_179"}
// Dependencies: {}
impl < A , F , Req , E > Future for MapInitErrFuture < A , F , Req , E > where A : ServiceFactory < Req > , F : Fn (A :: InitError) -> E , { type Output = Result < A :: Service , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; this . fut . poll (cx) . map_err (this . f) } }
};
}
