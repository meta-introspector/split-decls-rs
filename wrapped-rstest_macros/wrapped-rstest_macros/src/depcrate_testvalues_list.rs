// Generated macro for values_list (function)
macro_rules! Depcrate_testvalues_list {
() => {
// Module: crate::test
// Provides: {"values_list"}
// Dependencies: {}
pub (crate) fn values_list < S : AsRef < str > > (arg : & str , values : & [S]) -> ValueList { ValueList { arg : pat (arg) , values : values . into_iter () . map (| s | expr (s) . into ()) . collect () , } }
};
}
