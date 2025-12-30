// Generated macro for macro_15 (macro)
macro_rules! Depcrate_and_thenmacro_15 {
() => {
// Module: crate::and_then
// Provides: {"macro_15"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < A , B , Req > where A : Service < Req >, B : Service < A :: Response , Error = A :: Error >, { A { # [pin] fut : A :: Future , b : Option < Rc < (A , B) >>, } , B { # [pin] fut : B :: Future , } , } }
};
}
