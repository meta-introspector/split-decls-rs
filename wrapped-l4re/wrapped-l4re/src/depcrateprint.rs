// Generated macro for print (macro)
macro_rules! Depcrateprint {
() => {
// Module: crate
// Provides: {"print"}
// Dependencies: {}
macro_rules ! print { ($ ($ tt : tt) *) => { # [allow (unused_imports)] use core :: fmt :: Write as _ ; let _ = write ! (libc :: Stdout , $ ($ tt) *) ; } ; }
};
}
