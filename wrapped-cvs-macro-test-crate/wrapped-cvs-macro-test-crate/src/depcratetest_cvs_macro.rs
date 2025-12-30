// Generated macro for test_cvs_macro (function)
macro_rules! Depcratetest_cvs_macro {
() => {
// Module: crate
// Provides: {"test_cvs_macro"}
// Dependencies: {}
fn test_cvs_macro () { let empty_list : Cow < 'static , [Cow < 'static , str >] > = cvs ! () ; assert ! (empty_list . is_empty ()) ; let single_item : Cow < 'static , [Cow < 'static , str >] > = cvs ! ("one") ; assert_eq ! (single_item . len () , 1) ; assert_eq ! (single_item [0] , "one") ; let multiple_items : Cow < 'static , [Cow < 'static , str >] > = cvs ! ("apple" , "banana" , "cherry") ; assert_eq ! (multiple_items . len () , 3) ; assert_eq ! (multiple_items [0] , "apple") ; assert_eq ! (multiple_items [1] , "banana") ; assert_eq ! (multiple_items [2] , "cherry") ; println ! ("cvs! macro tests passed!") ; }
};
}
