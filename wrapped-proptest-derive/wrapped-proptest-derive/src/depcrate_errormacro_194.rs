// Generated macro for macro_194 (macro)
macro_rules! Depcrate_errormacro_194 {
() => {
// Module: crate::error
// Provides: {"macro_194"}
// Dependencies: {}
error ! (illegal_regex (item : & str) , E0007 , "`#[proptest(regex = \"<string>\")]` is not allowed on {0}. Only struct \
     fields, enum variant fields can use an explicit regex." , item) ;
};
}
