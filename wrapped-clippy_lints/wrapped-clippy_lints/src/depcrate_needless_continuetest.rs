// Generated macro for test (module)
macro_rules! Depcrate_needless_continuetest {
() => {
// Module: crate::needless_continue
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: erode_from_back ; # [test] # [rustfmt :: skip] fn test_erode_from_back () { let input = "\
{
    let x = 5;
    let y = format!(\"{}\", 42);
}" ; let expected = "\
{
    let x = 5;
    let y = format!(\"{}\", 42);" ; let got = erode_from_back (input) ; assert_eq ! (expected , got) ; } # [test] # [rustfmt :: skip] fn test_erode_from_back_no_brace () { let input = "\
let x = 5;
let y = something();
" ; let expected = input ; let got = erode_from_back (input) ; assert_eq ! (expected , got) ; } }
};
}
