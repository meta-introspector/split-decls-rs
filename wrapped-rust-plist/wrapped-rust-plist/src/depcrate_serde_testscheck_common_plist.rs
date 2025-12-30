// Generated macro for check_common_plist (function)
macro_rules! Depcrate_serde_testscheck_common_plist {
() => {
// Module: crate::serde_tests
// Provides: {"check_common_plist"}
// Dependencies: {}
fn check_common_plist (dict : & Dictionary) { let lines = dict . get ("Lines") . unwrap () . as_array () . unwrap () ; assert_eq ! (lines . len () , 2) ; assert_eq ! (lines [0] . as_string () . unwrap () , "It is a tale told by an idiot,     ") ; assert_eq ! (lines [1] . as_string () . unwrap () , "Full of sound and fury, signifying nothing.") ; assert ! (dict . get ("IsTrue") . unwrap () . as_boolean () . unwrap ()) ; assert ! (! dict . get ("IsNotFalse") . unwrap () . as_boolean () . unwrap ()) ; let data = dict . get ("Data") . unwrap () . as_data () . unwrap () ; assert_eq ! (data . len () , 15) ; assert_eq ! (data , & [0 , 0 , 0 , 0xbe , 0 , 0 , 0 , 0x03 , 0 , 0 , 0 , 0x1e , 0 , 0 , 0]) ; assert_eq ! (dict . get ("Birthdate") . unwrap () . as_date () . unwrap () , Date :: from_xml_format ("1981-05-16T11:32:06Z") . unwrap ()) ; assert_eq ! (dict . get ("Height") . unwrap () . as_real () . unwrap () , 1.6) ; assert_eq ! (dict . get ("BiggestNumber") . unwrap () . as_unsigned_integer () . unwrap () , 18446744073709551615) ; assert_eq ! (dict . get ("Death") . unwrap () . as_unsigned_integer () . unwrap () , 1564) ; assert_eq ! (dict . get ("SmallestNumber") . unwrap () . as_signed_integer () . unwrap () , - 9223372036854775808) ; assert_eq ! (dict . get ("Author") . unwrap () . as_string () . unwrap () , "William Shakespeare") ; assert_eq ! (dict . get ("Blank") . unwrap () . as_string () . unwrap () , "") ; }
};
}
