// Generated macro for lstm_predict (function)
macro_rules! Depcrate_unsflstm_predict {
() => {
// Module: crate::unsf
// Provides: {"lstm_predict"}
// Dependencies: {}
unsafe fn lstm_predict (l : usize , b : usize , w : * const f64 , w2 : * const f64 , s : * mut f64 , x : * const f64 , x2 : * mut f64 ,) { for i in 0 .. b { * x2 . add (i) = * x . add (i) * * w2 . add (i) ; } let mut xp = x2 ; let stop = 2 * l * b ; for i in (0 ..= stop - 1) . step_by (2 * b) { lstm_model (b , w . add (i * 4) , w . add ((i + b) * 4) , s . add (i) , s . add (i + b) , xp) ; xp = s . add (i) ; } for i in 0 .. b { * x2 . add (i) = * xp . add (i) * * w2 . add (b + i) + * w2 . add (2 * b + i) ; } }
};
}
