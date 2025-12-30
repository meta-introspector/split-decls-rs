// Generated macro for macro_213 (macro)
macro_rules! Depcrate_errormacro_213 {
() => {
// Module: crate::error
// Provides: {"macro_213"}
// Dependencies: {}
error ! (strategy_malformed (meta : & syn :: Meta) , E0026 , "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
     format `#[proptest({0} = \"<expr>\")]` where `<expr>` is a valid Rust \
     expression." , meta . path () . into_token_stream ()) ;
};
}
