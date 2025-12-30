// Generated macro for concatenate_test (module)
macro_rules! Depcrate_common_descriptionconcatenate_test {
() => {
// Module: crate::common::description
// Provides: {"concatenate_test"}
// Dependencies: {}
# [cfg (test)] mod concatenate_test { use super :: Description ; # [doc = " Forms a [`Vec`] of [`String`]s out of the provided [`str`]s"] # [doc = " [`Iterator`]."] fn to_strings < 'i > (source : impl IntoIterator < Item = & 'i str >) -> Vec < String > { source . into_iter () . map (Into :: into) . collect () } # [test] fn single () { assert_eq ! (Description :: concatenate (& to_strings (["foo"])) , "foo") ; } # [test] fn multiple () { assert_eq ! (Description :: concatenate (& to_strings (["foo" , "bar"])) , "foo\nbar" ,) ; } # [test] fn trims_spaces () { assert_eq ! (Description :: concatenate (& to_strings ([" foo " , "bar " , " baz"])) , "foo\nbar\nbaz" ,) ; } # [test] fn empty () { assert_eq ! (Description :: concatenate (& to_strings (["foo" , "" , "bar"])) , "foo\n\nbar" ,) ; } # [test] fn newline_spaces () { assert_eq ! (Description :: concatenate (& to_strings (["foo " , "" , " bar"])) , "foo\n\nbar" ,) ; } # [test] fn continuation_backslash () { assert_eq ! (Description :: concatenate (& to_strings (["foo\\" , "x\\" , "y" , "bar"])) , "foo x y\nbar" ,) ; } }
};
}
