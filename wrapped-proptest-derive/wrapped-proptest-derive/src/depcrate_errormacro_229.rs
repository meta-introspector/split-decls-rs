// Generated macro for macro_229 (macro)
macro_rules! Depcrate_errormacro_229 {
() => {
// Module: crate::error
// Provides: {"macro_229"}
// Dependencies: {}
error ! (cant_set_param_and_regex (item : & str) , E0035 , "Cannot set #[proptest(regex = \"<string>\")] and \
     `#[proptest(params = <type>)]` on {0} because the latter is a logic bug \
     since `params` cannot be used in `<string>`." , item) ;
};
}
