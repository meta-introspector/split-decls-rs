// Generated macro for macro_21 (macro)
macro_rules! Depcrate_and_thenmacro_21 {
() => {
// Module: crate::and_then
// Provides: {"macro_21"}
// Dependencies: {}
pin_project ! { pub struct AndThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req >, B : ServiceFactory < A :: Response >, { # [pin] fut_a : A :: Future , # [pin] fut_b : B :: Future , a : Option < A :: Service >, b : Option < B :: Service >, } }
};
}
