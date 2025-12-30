// Generated macro for get_workspace (function)
macro_rules! Depcrate_safeget_workspace {
() => {
// Module: crate::safe
// Provides: {"get_workspace"}
// Dependencies: {}
fn get_workspace (d : usize , k : usize) -> (Vec < f64 > , Vec < f64 > , Vec < f64 > , Vec < f64 > , Vec < f64 >) { let qdiags = vec ! [0. ; d * k] ; let sum_qs = vec ! [0. ; k] ; let xcentered = vec ! [0. ; d] ; let qxcentered = vec ! [0. ; d] ; let main_term = vec ! [0. ; k] ; (qdiags , sum_qs , xcentered , qxcentered , main_term) }
};
}
