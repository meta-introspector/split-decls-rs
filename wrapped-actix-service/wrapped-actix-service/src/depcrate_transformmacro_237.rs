// Generated macro for macro_237 (macro)
macro_rules! Depcrate_transformmacro_237 {
() => {
// Module: crate::transform
// Provides: {"macro_237"}
// Dependencies: {}
pin_project ! { pub struct ApplyTransformFuture < T , S , Req > where S : ServiceFactory < Req >, T : Transform < S :: Service , Req , InitError = S :: InitError >, { store : Rc < (T , S) >, # [pin] state : ApplyTransformFutureState < T , S , Req >, } }
};
}
