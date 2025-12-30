// Generated macro for test_attributes_single (function)
macro_rules! Depcrate_utilstest_attributes_single {
() => {
// Module: crate::utils
// Provides: {"test_attributes_single"}
// Dependencies: {}
# [test] fn test_attributes_single () { for attr in Attribute :: MAP { let attrs = Attributes :: new () . insert (attr) ; assert_eq ! (attrs . bits () . collect ::< Vec < _ >> () , [attr as u16]) ; assert_eq ! (attrs . attrs () . collect ::< Vec < _ >> () , [attr]) ; assert_eq ! (format ! ("{attrs:?}") , format ! ("{{{:?}}}" , attr)) ; } }
};
}
