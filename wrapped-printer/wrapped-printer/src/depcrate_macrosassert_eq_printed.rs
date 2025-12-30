// Generated macro for assert_eq_printed (macro)
macro_rules! Depcrate_macrosassert_eq_printed {
() => {
// Module: crate::macros
// Provides: {"assert_eq_printed"}
// Dependencies: {}
# [doc = " Like assert_eq, but nicer output for long strings."] # [cfg (test)] # [macro_export] macro_rules ! assert_eq_printed { ($ expected : expr , $ got : expr) => { let expected = &*$ expected ; let got = &*$ got ; if expected != got { panic ! ("
printed outputs differ!

expected:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

got:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
{}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
" , expected , got) ; } } }
};
}
