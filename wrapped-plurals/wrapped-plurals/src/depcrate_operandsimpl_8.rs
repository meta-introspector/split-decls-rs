// Generated macro for impl_8 (impl)
macro_rules! Depcrate_operandsimpl_8 {
() => {
// Module: crate::operands
// Provides: {"impl_8"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl core :: str :: FromStr for PluralOperands { type Err = OperandsError ; fn from_str (input : & str) -> Result < Self , Self :: Err > { fn get_exponent (input : & str) -> Result < (& str , usize) , OperandsError > { if let Some ((base , exponent)) = input . split_once ('e') { Ok ((base , exponent . parse () ?)) } else { Ok ((input , 0)) } } if input . is_empty () { return Err (OperandsError :: Empty) ; } let abs_str = input . strip_prefix ('-') . unwrap_or (input) ; let (integer_digits , num_fraction_digits0 , num_fraction_digits , fraction_digits0 , fraction_digits , exponent ,) = if let Some ((int_str , rest)) = abs_str . split_once ('.') { let (dec_str , exponent) = get_exponent (rest) ? ; let integer_digits = u64 :: from_str (int_str) ? ; let dec_str_no_zeros = dec_str . trim_end_matches ('0') ; let num_fraction_digits0 = dec_str . len () ; let num_fraction_digits = dec_str_no_zeros . len () ; let fraction_digits0 = u64 :: from_str (dec_str) ? ; let fraction_digits = if num_fraction_digits == 0 || num_fraction_digits == num_fraction_digits0 { fraction_digits0 } else { u64 :: from_str (dec_str_no_zeros) ? } ; (integer_digits , num_fraction_digits0 , num_fraction_digits , fraction_digits0 , fraction_digits , exponent ,) } else { let (abs_str , exponent) = get_exponent (abs_str) ? ; let integer_digits = u64 :: from_str (abs_str) ? ; (integer_digits , 0 , 0 , 0 , 0 , exponent) } ; Ok (Self { i : integer_digits , v : num_fraction_digits0 , w : num_fraction_digits , f : fraction_digits0 , t : fraction_digits , c : exponent , }) } }
};
}
