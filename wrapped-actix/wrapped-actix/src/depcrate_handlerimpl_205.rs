// Generated macro for impl_205 (impl)
macro_rules! Depcrate_handlerimpl_205 {
() => {
// Module: crate::handler
// Provides: {"impl_205"}
// Dependencies: {}
impl < A , M , I , E > MessageResponse < A , M > for Result < I , E > where A : Actor , M : Message < Result = Self > , I : 'static , E : 'static , { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender < Self > >) { tx . send (self) } }
};
}
