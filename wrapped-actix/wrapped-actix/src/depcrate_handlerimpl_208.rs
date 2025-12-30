// Generated macro for impl_208 (impl)
macro_rules! Depcrate_handlerimpl_208 {
() => {
// Module: crate::handler
// Provides: {"impl_208"}
// Dependencies: {}
impl < A , M , I > MessageResponse < A , M > for Vec < I > where A : Actor , M : Message < Result = Self > , I : 'static , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < Self > >) { tx . send (self) } }
};
}
