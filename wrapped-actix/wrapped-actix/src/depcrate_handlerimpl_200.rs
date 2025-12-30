// Generated macro for impl_200 (impl)
macro_rules! Depcrate_handlerimpl_200 {
() => {
// Module: crate::handler
// Provides: {"impl_200"}
// Dependencies: {}
impl < A , M > MessageResponse < A , M > for AtomicResponse < A , M :: Result > where A : Actor , M : Message , A :: Context : AsyncContext < A > , { fn handle (self , ctx : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) { ctx . wait (self . 0 . map (| res , _ , _ | tx . send (res))) ; } }
};
}
