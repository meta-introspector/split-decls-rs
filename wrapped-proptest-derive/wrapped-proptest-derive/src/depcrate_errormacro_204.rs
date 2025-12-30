// Generated macro for macro_204 (macro)
macro_rules! Depcrate_errormacro_204 {
() => {
// Module: crate::error
// Provides: {"macro_204"}
// Dependencies: {}
error ! (set_again (meta : & syn :: Meta) , E0017 , "The attribute modifier `{}` inside `#[proptest(..)]` has already been \
     set. To fix the error, please remove at least one such modifier." , meta . path () . into_token_stream ()) ;
};
}
