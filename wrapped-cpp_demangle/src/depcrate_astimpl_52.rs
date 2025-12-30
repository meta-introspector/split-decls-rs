// Generated macro for impl_52 (impl)
macro_rules! Depcrate_astimpl_52 {
() => {
// Module: crate::ast
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a , 'b , W : 'a + DemangleWrite > AutoParseDemangle < 'a , 'b , W > { # [inline] fn new (ctx : & 'b mut DemangleContext < 'a , W >) -> core :: result :: Result < Self , fmt :: Error > { ctx . enter_recursion () ? ; Ok (AutoParseDemangle (ctx)) } }
};
}
