// Generated macro for impl_writeable_for_key_value (macro)
macro_rules! Depcrate_helpersimpl_writeable_for_key_value {
() => {
// Module: crate::helpers
// Provides: {"impl_writeable_for_key_value"}
// Dependencies: {}
macro_rules ! impl_writeable_for_key_value { ($ type : tt , $ key1 : literal , $ value1 : literal , $ key2 : literal , $ expected2 : literal) => { impl_writeable_for_each_subtag_str_no_test ! ($ type) ; # [test] fn test_writeable () { writeable :: assert_writeable_eq ! (&$ type :: default () , "") ; writeable :: assert_writeable_eq ! (&$ type :: from_tuple_vec (vec ! [($ key1 . parse () . unwrap () , $ value1 . parse () . unwrap ())]) , core :: concat ! ($ key1 , "-" , $ value1) ,) ; writeable :: assert_writeable_eq ! (&$ type :: from_tuple_vec (vec ! [($ key1 . parse () . unwrap () , $ value1 . parse () . unwrap ()) , ($ key2 . parse () . unwrap () , "true" . parse () . unwrap ())]) , core :: concat ! ($ key1 , "-" , $ value1 , "-" , $ expected2) ,) ; } } ; }
};
}
