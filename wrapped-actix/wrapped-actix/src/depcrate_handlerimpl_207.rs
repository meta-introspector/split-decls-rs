// Generated macro for impl_207 (impl)
macro_rules! Depcrate_handlerimpl_207 {
() => {
// Module: crate::handler
// Provides: {"impl_207"}
// Dependencies: {}
impl < A , M , I > MessageResponse < A , M > for Option < I > where A : Actor , M : Message < Result = Self > , I : 'static , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < Self > >) { tx . send (self) } }
};
}
