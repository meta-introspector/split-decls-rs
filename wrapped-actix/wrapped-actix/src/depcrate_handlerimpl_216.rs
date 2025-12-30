// Generated macro for impl_216 (impl)
macro_rules! Depcrate_handlerimpl_216 {
() => {
// Module: crate::handler
// Provides: {"impl_216"}
// Dependencies: {}
impl < A , M > MessageResponse < A , M > for Response < M :: Result > where A : Actor , M : Message , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) { match self . item { ResponseTypeItem :: Fut (fut) => { actix_rt :: spawn (async { tx . send (fut . await) }) ; } ResponseTypeItem :: Result (res) => tx . send (res) , } } }
};
}
