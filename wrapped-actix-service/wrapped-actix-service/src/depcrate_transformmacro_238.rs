// Generated macro for macro_238 (macro)
macro_rules! Depcrate_transformmacro_238 {
() => {
// Module: crate::transform
// Provides: {"macro_238"}
// Dependencies: {}
pin_project ! { # [project = ApplyTransformFutureStateProj] pub enum ApplyTransformFutureState < T , S , Req > where S : ServiceFactory < Req >, T : Transform < S :: Service , Req , InitError = S :: InitError >, { A { # [pin] fut : S :: Future } , B { # [pin] fut : T :: Future } , } }
};
}
