// Generated macro for macro_209 (macro)
macro_rules! Depcrate_errormacro_209 {
() => {
// Module: crate::error
// Provides: {"macro_209"}
// Dependencies: {}
error ! (weight_malformed (meta : & syn :: Meta) , E0021 , "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
    format `#[proptest({0} = <integer>)]` where `<integer>` is an integer that \
    fits within a `u32`. An example: `#[proptest({0} = 2)]` to set a relative \
    weight of 2." , meta . path () . into_token_stream ()) ;
};
}
