// Generated macro for test_bitfields_seventh (function)
macro_rules! Depcratetest_bitfields_seventh {
() => {
// Module: crate
// Provides: {"test_bitfields_seventh"}
// Dependencies: {}
# [test] fn test_bitfields_seventh () { let mut large : bindings :: bitfields :: Seventh = unsafe { mem :: zeroed () } ; assert ! (unsafe { large . assert (false , 0 , 0 , 0 , 0 , false , 0) }) ; large . set_first_one_bit (true) ; large . set_second_thirty_bits (375028802) ; large . set_third_two_bits (2) ; large . set_fourth_thirty_bits (643472885) ; large . set_fifth_two_bits (3) ; large . set_sixth_one_bit (true) ; large . set_seventh_thirty_bits (1061657575) ; assert ! (unsafe { large . assert (true , 375028802 , 2 , 643472885 , 3 , true , 1061657575) }) ; assert_eq ! (large . first_one_bit () , true) ; assert_eq ! (large . second_thirty_bits () , 375028802) ; assert_eq ! (large . third_two_bits () , 2) ; assert_eq ! (large . fourth_thirty_bits () , 643472885) ; assert_eq ! (large . fifth_two_bits () , 3) ; assert_eq ! (large . sixth_one_bit () , true) ; assert_eq ! (large . seventh_thirty_bits () , 1061657575) ; }
};
}
