// Generated macro for foo (function)
macro_rules! Depcratefoo {
() => {
// Module: crate
// Provides: {"foo"}
// Dependencies: {}
# [unsafe (no_mangle)] pub extern "C" fn foo (outer : Pos , inner : fn (Pos , Pos)) { inner (outer , pos ! ()) ; }
};
}
