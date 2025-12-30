// Generated macro for float_to_string (function)
macro_rules! Depcrate_data_floatfloat_to_string {
() => {
// Module: crate::data::float
// Provides: {"float_to_string"}
// Dependencies: {}
# [allow (clippy :: never_loop)] fn float_to_string (n : f64 , max_precision : usize , min_decimal : usize) -> String { let (mut result , mut count) = loop { let (sign , n) = if n < 0.0 { ("-" , - n) } else { ("" , n) } ; let int_part = n . floor () ; let dec_part = ((n . abs () - int_part . abs ()) * (10.0f64) . powi (max_precision as i32)) . round () as u64 ; if dec_part == 0 || max_precision == 0 { break (format ! ("{}{:.0}" , sign , int_part) , 0) ; } let mut leading = "" . to_string () ; let mut dec_result = format ! ("{}" , dec_part) ; for _ in 0 .. (max_precision - dec_result . len ()) { leading . push ('0') ; } while let Some (c) = dec_result . pop () { if c != '0' { dec_result . push (c) ; break ; } } break (format ! ("{}{:.0}.{}{}" , sign , int_part , leading , dec_result) , leading . len () + dec_result . len () ,) ; } ; if count == 0 && min_decimal > 0 { result . push ('.') ; } while count < min_decimal { result . push ('0') ; count += 1 ; } result }
};
}
