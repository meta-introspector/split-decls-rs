// Generated macro for impl_210 (impl)
macro_rules! Depcrate_handlerimpl_210 {
() => {
// Module: crate::handler
// Provides: {"impl_210"}
// Dependencies: {}
impl < A , M > MessageResponse < A , M > for ResponseActFuture < A , M :: Result > where A : Actor , M : Message , A :: Context : AsyncContext < A > , { fn handle (self , ctx : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) { ctx . spawn (self . map (| res , _ , _ | tx . send (res))) ; } }
};
}
