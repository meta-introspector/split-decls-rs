// Generated macro for tests (module)
macro_rules! Depcrate_numbertests {
() => {
// Module: crate::number
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "std")] mod tests { use super :: * ; use crate :: error :: ErrorKind ; use crate :: internal :: Err ; macro_rules ! assert_parse (($ left : expr , $ right : expr) => { let res : $ crate :: IResult < _ , _ , (_ , ErrorKind) > = $ left ; assert_eq ! (res , $ right) ; } ;) ; # [test] fn float_test () { let mut test_cases = vec ! ["+3.14" , "3.14" , "-3.14" , "0" , "0.0" , "1." , ".789" , "-.5" , "1e7" , "-1E-7" , ".3e-2" , "1.e4" , "1.2e4" , "12.34" , "-1.234E-12" , "-1.234e-12" , "0.00000000000000000087" ,] ; for test in test_cases . drain (..) { let expected32 = str :: parse :: < f32 > (test) . unwrap () ; let expected64 = str :: parse :: < f64 > (test) . unwrap () ; println ! ("now parsing: {} -> {}" , test , expected32) ; assert_parse ! (recognize_float () . parse_complete (test) , Ok (("" , test))) ; assert_parse ! (double () . parse_complete (test . as_bytes ()) , Ok ((& b"" [..] , expected64))) ; assert_parse ! (double () . parse_complete (test) , Ok (("" , expected64))) ; } let remaining_exponent = "-1.234E-" ; assert_parse ! (recognize_float () . parse_complete (remaining_exponent) , Err (Err :: Failure (("" , ErrorKind :: Digit)))) ; } }
};
}
