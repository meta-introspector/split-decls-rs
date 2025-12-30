// Generated macro for macro_213 (macro)
macro_rules! Depcrate_thenmacro_213 {
() => {
// Module: crate::then
// Provides: {"macro_213"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < A , B , Req > where A : Service < Req >, B : Service < Result < A :: Response , A :: Error >>, { A { # [pin] fut : A :: Future , b : Option < Rc < (A , B) >> } , B { # [pin] fut : B :: Future } , } }
};
}
