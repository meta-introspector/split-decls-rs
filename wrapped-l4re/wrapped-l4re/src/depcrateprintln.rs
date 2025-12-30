// Generated macro for println (macro)
macro_rules! Depcrateprintln {
() => {
// Module: crate
// Provides: {"println"}
// Dependencies: {}
macro_rules ! println { ($ ($ tt : tt) *) => { # [allow (unused_imports)] use core :: fmt :: Write as _ ; let _ = writeln ! (libc :: Stdout , $ ($ tt) *) ; } ; }
};
}
