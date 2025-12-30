// Generated macro for w (macro)
macro_rules! Depcrate_codepointtrie_cptriew {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"w"}
// Dependencies: {}
macro_rules ! w (($ a : tt + $ b : expr) => { { # [allow (unused_parens)] let a = $ a ; let b = $ b ; debug_assert ! (a . checked_add (b) . is_some ()) ; $ a . wrapping_add ($ b) } } ; ($ a : tt - $ b : expr) => { { # [allow (unused_parens)] let a = $ a ; let b = $ b ; debug_assert ! (a . checked_sub (b) . is_some ()) ; $ a . wrapping_sub ($ b) } } ; ($ a : tt * $ b : expr) => { { # [allow (unused_parens)] let a = $ a ; let b = $ b ; debug_assert ! (a . checked_mul (b) . is_some ()) ; $ a . wrapping_mul ($ b) } } ;) ;
};
}
