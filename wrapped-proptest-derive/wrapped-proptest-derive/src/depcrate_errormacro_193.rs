// Generated macro for macro_193 (macro)
macro_rules! Depcrate_errormacro_193 {
() => {
// Module: crate::error
// Provides: {"macro_193"}
// Dependencies: {}
error ! (illegal_strategy (attr : & str , item : & str) , E0007 , "`#[proptest({0} = \"<expr>\")]` is not allowed on {1}. Only struct fields, \
    enum variants and fields inside those can use an explicit {0}." , attr , item) ;
};
}
