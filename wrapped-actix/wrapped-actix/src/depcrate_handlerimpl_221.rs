// Generated macro for impl_221 (impl)
macro_rules! Depcrate_handlerimpl_221 {
() => {
// Module: crate::handler
// Provides: {"impl_221"}
// Dependencies: {}
impl < A , M > MessageResponse < A , M > for ActorResponse < A , M :: Result > where A : Actor , M : Message , A :: Context : AsyncContext < A > , { fn handle (self , ctx : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) { match self . item { ActorResponseTypeItem :: Fut (fut) => { let fut = fut . map (| res , _ , _ | tx . send (res)) ; ctx . spawn (fut) ; } ActorResponseTypeItem :: Result (res) => tx . send (res) , } } }
};
}
