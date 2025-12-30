// Generated macro for macro_212 (macro)
macro_rules! Depcrate_thenmacro_212 {
() => {
// Module: crate::then
// Provides: {"macro_212"}
// Dependencies: {}
pin_project ! { pub (crate) struct ThenServiceResponse < A , B , Req > where A : Service < Req >, B : Service < Result < A :: Response , A :: Error >>, { # [pin] state : State < A , B , Req >, } }
};
}
