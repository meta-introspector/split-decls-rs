// Generated macro for impl_writeable_for_subtag_list (macro)
macro_rules! Depcrate_helpersimpl_writeable_for_subtag_list {
() => {
// Module: crate::helpers
// Provides: {"impl_writeable_for_subtag_list"}
// Dependencies: {}
macro_rules ! impl_writeable_for_subtag_list { ($ type : tt , $ sample1 : literal , $ sample2 : literal) => { impl_writeable_for_each_subtag_str_no_test ! ($ type , selff , selff . 0 . len () == 1 => # [allow (clippy :: unwrap_used)] { Some (selff . 0 . get (0) . unwrap () . as_str ()) }) ; # [test] fn test_writeable () { writeable :: assert_writeable_eq ! (&$ type :: default () , "") ; writeable :: assert_writeable_eq ! (&$ type :: from_vec_unchecked (alloc :: vec ! [$ sample1 . parse () . unwrap ()]) , $ sample1 ,) ; writeable :: assert_writeable_eq ! (&$ type :: from_vec_unchecked (vec ! [$ sample1 . parse () . unwrap () , $ sample2 . parse () . unwrap ()]) , core :: concat ! ($ sample1 , "-" , $ sample2) ,) ; } } ; }
};
}
