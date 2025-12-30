// Generated macro for lstm_model (function)
macro_rules! Depcrate_safelstm_model {
() => {
// Module: crate::safe
// Provides: {"lstm_model"}
// Dependencies: {}
fn lstm_model (hsize : usize , weight : & [f64] , bias : & [f64] , hidden : & mut [f64] , cell : & mut [f64] , input : & [f64] ,) { let mut gates = vec ! [0.0 ; 4 * hsize] ; let gates = & mut gates [.. 4 * hsize] ; let (a , b) = gates . split_at_mut (2 * hsize) ; let ((forget , ingate) , (outgate , change)) = (a . split_at_mut (hsize) , b . split_at_mut (hsize)) ; for i in 0 .. hsize { forget [i] = sigmoid (input [i] * weight [i] + bias [i]) ; ingate [i] = sigmoid (hidden [i] * weight [hsize + i] + bias [hsize + i]) ; outgate [i] = sigmoid (input [i] * weight [2 * hsize + i] + bias [2 * hsize + i]) ; change [i] = (hidden [i] * weight [3 * hsize + i] + bias [3 * hsize + i]) . tanh () ; } for i in 0 .. hsize { cell [i] = cell [i] * forget [i] + ingate [i] * change [i] ; } for i in 0 .. hsize { hidden [i] = outgate [i] * cell [i] . tanh () ; } }
};
}
