// Generated macro for lstm_predict (function)
macro_rules! Depcrate_safelstm_predict {
() => {
// Module: crate::safe
// Provides: {"lstm_predict"}
// Dependencies: {}
fn lstm_predict (l : usize , b : usize , w : & [f64] , w2 : & [f64] , s : & mut [f64] , x : & [f64] , x2 : & mut [f64] ,) { for i in 0 .. b { x2 [i] = x [i] * w2 [i] ; } let mut i = 0 ; while i <= 2 * l * b - 1 { let (xp , s1 , s2) = if i == 0 { let (s1 , s2) = s . split_at_mut (b) ; (x2 . as_mut () , s1 , s2) } else { let tmp = & mut s [i - 2 * b ..] ; let (a , d) = tmp . split_at_mut (2 * b) ; let (d , c) = d . split_at_mut (b) ; (a , d , c) } ; lstm_model (b , & w [i * 4 .. (i + b) * 4] , & w [(i + b) * 4 .. (i + 2 * b) * 4] , s1 , s2 , xp ,) ; i += 2 * b ; } let xp = & s [i - 2 * b ..] ; for i in 0 .. b { x2 [i] = xp [i] * w2 [b + i] + w2 [2 * b + i] ; } }
};
}
