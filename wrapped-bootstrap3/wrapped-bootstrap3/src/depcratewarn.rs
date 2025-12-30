// Generated macro for warn (macro)
macro_rules! Depcratewarn {
() => {
// Module: crate
// Provides: {"warn"}
// Dependencies: {}
macro_rules ! warn { ($ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: warn! macro called") ; println ! ($ ($ tt) *) } ; }
};
}
