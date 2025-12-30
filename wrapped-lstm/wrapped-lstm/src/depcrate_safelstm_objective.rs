// Generated macro for lstm_objective (function)
macro_rules! Depcrate_safelstm_objective {
() => {
// Module: crate::safe
// Provides: {"lstm_objective"}
// Dependencies: {}
# [autodiff (d_lstm_objective , Reverse , Const , Const , Const , Duplicated , Duplicated , Const , Const , DuplicatedOnly)] pub (crate) fn lstm_objective (l : usize , c : usize , b : usize , main_params : & [f64] , extra_params : & [f64] , state : & mut [f64] , sequence : & [f64] , loss : & mut f64 ,) { let mut total = 0.0 ; let mut input = & sequence [.. b] ; let mut ypred = vec ! [0.0 ; b] ; let mut ynorm = vec ! [0.0 ; b] ; let limit = (c - 1) * b ; for j in 0 .. (c - 1) { let t = j * b ; lstm_predict (l , b , main_params , extra_params , state , input , & mut ypred) ; let lse = logsumexp (& ypred) ; for i in 0 .. b { ynorm [i] = ypred [i] - lse ; } let ygold = & sequence [t + b ..] ; for i in 0 .. b { total += ygold [i] * ynorm [i] ; } input = ygold ; } let count = (c - 1) * b ; * loss = - total / count as f64 ; }
};
}
