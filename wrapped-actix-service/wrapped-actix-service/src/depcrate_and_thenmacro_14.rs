// Generated macro for macro_14 (macro)
macro_rules! Depcrate_and_thenmacro_14 {
() => {
// Module: crate::and_then
// Provides: {"macro_14"}
// Dependencies: {}
pin_project ! { pub struct AndThenServiceResponse < A , B , Req > where A : Service < Req >, B : Service < A :: Response , Error = A :: Error >, { # [pin] state : State < A , B , Req >, } }
};
}
