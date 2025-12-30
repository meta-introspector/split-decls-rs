// Generated macro for macro_219 (macro)
macro_rules! Depcrate_thenmacro_219 {
() => {
// Module: crate::then
// Provides: {"macro_219"}
// Dependencies: {}
pin_project ! { pub (crate) struct ThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req >, B : ServiceFactory < Result < A :: Response , A :: Error >, Config = A :: Config , Error = A :: Error , InitError = A :: InitError , >, { # [pin] fut_b : B :: Future , # [pin] fut_a : A :: Future , a : Option < A :: Service >, b : Option < B :: Service >, } }
};
}
