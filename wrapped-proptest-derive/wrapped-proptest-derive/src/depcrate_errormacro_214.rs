// Generated macro for macro_214 (macro)
macro_rules! Depcrate_errormacro_214 {
() => {
// Module: crate::error
// Provides: {"macro_214"}
// Dependencies: {}
error ! (filter_malformed (meta : & syn :: Meta) , E0027 , "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
     format `#[proptest({0} = \"<expr>\")]` where `<expr>` is a valid Rust \
     expression." , meta . path () . into_token_stream ()) ;
};
}
