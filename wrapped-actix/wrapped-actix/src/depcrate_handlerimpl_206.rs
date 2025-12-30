// Generated macro for impl_206 (impl)
macro_rules! Depcrate_handlerimpl_206 {
() => {
// Module: crate::handler
// Provides: {"impl_206"}
// Dependencies: {}
impl < A , M , I > MessageResponse < A , M > for Arc < I > where A : Actor , M : Message < Result = Self > , I : 'static , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < Self > >) { tx . send (self) } }
};
}
