// Generated macro for impl_606 (impl)
macro_rules! Depcrate_process_results_implimpl_606 {
() => {
// Module: crate::process_results_impl
// Provides: {"impl_606"}
// Dependencies: {}
impl < I , E > ProcessResults < '_ , I , E > { # [inline (always)] fn next_body < T > (& mut self , item : Option < Result < T , E > >) -> Option < T > { match item { Some (Ok (x)) => Some (x) , Some (Err (e)) => { * self . error = Err (e) ; None } None => None , } } }
};
}
