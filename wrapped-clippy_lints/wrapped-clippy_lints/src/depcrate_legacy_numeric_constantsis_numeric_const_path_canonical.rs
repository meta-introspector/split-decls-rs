// Generated macro for is_numeric_const_path_canonical (function)
macro_rules! Depcrate_legacy_numeric_constantsis_numeric_const_path_canonical {
() => {
// Module: crate::legacy_numeric_constants
// Provides: {"is_numeric_const_path_canonical"}
// Dependencies: {}
fn is_numeric_const_path_canonical (expr_path : & hir :: Path < '_ > , [mod_name , name] : [Symbol ; 2]) -> bool { let [hir :: PathSegment { ident : one , args : None , .. } , hir :: PathSegment { ident : two , args : None , .. } ,] = expr_path . segments else { return false ; } ; one . name == mod_name && two . name == name }
};
}
