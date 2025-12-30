// Generated macro for impl_143 (impl)
macro_rules! Depcrate_stackimpl_143 {
() => {
// Module: crate::stack
// Provides: {"impl_143"}
// Dependencies: {}
impl < A , R , Closure : Clone > Clone for StackBlock < '_ , A , R , Closure > { # [inline] fn clone (& self) -> Self { Self { p : PhantomData , header : self . header , closure : self . closure . clone () , } } }
};
}
