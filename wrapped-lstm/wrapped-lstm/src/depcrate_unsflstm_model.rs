// Generated macro for lstm_model (function)
macro_rules! Depcrate_unsflstm_model {
() => {
// Module: crate::unsf
// Provides: {"lstm_model"}
// Dependencies: {}
unsafe fn lstm_model (hsize : usize , weight : * const f64 , bias : * const f64 , hidden : * mut f64 , cell : * mut f64 , input : * const f64 ,) { let mut gates = vec ! [0.0 ; 4 * hsize] ; let forget : * mut f64 = gates . as_mut_ptr () ; let ingate : * mut f64 = gates [hsize ..] . as_mut_ptr () ; let outgate : * mut f64 = gates [2 * hsize ..] . as_mut_ptr () ; let change : * mut f64 = gates [3 * hsize ..] . as_mut_ptr () ; for i in 0 .. hsize { * forget . add (i) = sigmoid (* input . add (i) * * weight . add (i) + * bias . add (i)) ; * ingate . add (i) = sigmoid (* hidden . add (i) * * weight . add (hsize + i) + * bias . add (hsize + i)) ; * outgate . add (i) = sigmoid (* input . add (i) * * weight . add (2 * hsize + i) + * bias . add (2 * hsize + i)) ; * change . add (i) = (* hidden . add (i) * * weight . add (3 * hsize + i) + * bias . add (3 * hsize + i)) . tanh () ; } for i in 0 .. hsize { * cell . add (i) = * cell . add (i) * * forget . add (i) + * ingate . add (i) * * change . add (i) ; } for i in 0 .. hsize { * hidden . add (i) = * outgate . add (i) * (* cell . add (i)) . tanh () ; } }
};
}
