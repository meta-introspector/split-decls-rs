// Generated macro for impl_209 (impl)
macro_rules! Depcrate_handlerimpl_209 {
() => {
// Module: crate::handler
// Provides: {"impl_209"}
// Dependencies: {}
impl < A , M , B > MessageResponse < A , M > for Addr < B > where A : Actor , M : Message < Result = Self > , B : Actor , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < Self > >) { tx . send (self) } }
};
}
