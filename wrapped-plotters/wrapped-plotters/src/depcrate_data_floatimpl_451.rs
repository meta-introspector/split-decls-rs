// Generated macro for impl_451 (impl)
macro_rules! Depcrate_data_floatimpl_451 {
() => {
// Module: crate::data::float
// Provides: {"impl_451"}
// Dependencies: {}
impl FloatPrettyPrinter { # [doc = " Handles printing of floating point numbers"] pub fn print (& self , n : f64) -> String { let (tn , p) = find_minimal_repr (n , (10f64) . powi (- self . max_decimal)) ; let d_repr = float_to_string (tn , p , self . min_decimal as usize) ; if ! self . allow_scientific { d_repr } else { if n == 0.0 { return "0" . to_string () ; } let mut idx = n . abs () . log10 () . floor () ; let mut exp = (10.0f64) . powf (idx) ; if n . abs () / exp + 1e-5 >= 10.0 { idx += 1.0 ; exp *= 10.0 ; } if idx . abs () < 3.0 { return d_repr ; } let (sn , sp) = find_minimal_repr (n / exp , 1e-5) ; let s_repr = format ! ("{}e{}" , float_to_string (sn , sp , self . min_decimal as usize) , float_to_string (idx , 0 , 0)) ; if s_repr . len () + 1 < d_repr . len () || (tn == 0.0 && n != 0.0) { s_repr } else { d_repr } } } }
};
}
