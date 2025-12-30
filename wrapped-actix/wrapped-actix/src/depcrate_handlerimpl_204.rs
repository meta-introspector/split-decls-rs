// Generated macro for impl_204 (impl)
macro_rules! Depcrate_handlerimpl_204 {
() => {
// Module: crate::handler
// Provides: {"impl_204"}
// Dependencies: {}
impl < A , M > MessageResponse < A , M > for MessageResult < M > where A : Actor , M : Message , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) { tx . send (self . 0) } }
};
}
