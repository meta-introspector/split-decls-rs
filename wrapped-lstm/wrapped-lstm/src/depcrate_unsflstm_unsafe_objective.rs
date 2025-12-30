// Generated macro for lstm_unsafe_objective (function)
macro_rules! Depcrate_unsflstm_unsafe_objective {
() => {
// Module: crate::unsf
// Provides: {"lstm_unsafe_objective"}
// Dependencies: {}
# [autodiff (d_lstm_unsafe_objective , Reverse , Const , Const , Const , Duplicated , Duplicated , Const , Const , DuplicatedOnly)] pub (crate) unsafe fn lstm_unsafe_objective (l : usize , c : usize , b : usize , main_params : * const f64 , extra_params : * const f64 , state : * mut f64 , sequence : * const f64 , loss : * mut f64) { let mut total = 0.0 ; let mut count = 0 ; let mut input = sequence ; let mut ypred = vec ! [0.0 ; b] ; let mut ynorm = vec ! [0.0 ; b] ; let mut lse ; assert ! (b > 0) ; let stop = (c - 1) * b ; for t in (0 ..= stop - 1) . step_by (b) { lstm_predict (l , b , main_params , extra_params , state , input , ypred . as_mut_ptr ()) ; lse = logsumexp (ypred . as_mut_ptr () , b) ; for i in 0 .. b { ynorm [i] = ypred [i] - lse ; } let ygold = sequence . add (t + b) ; for i in 0 .. b { total += * ygold . add (i) * ynorm [i] ; } count += b ; input = ygold ; } * loss = - total / count as f64 ; }
};
}
