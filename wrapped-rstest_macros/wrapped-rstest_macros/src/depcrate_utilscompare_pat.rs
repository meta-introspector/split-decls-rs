// Generated macro for compare_pat (function)
macro_rules! Depcrate_utilscompare_pat {
() => {
// Module: crate::utils
// Provides: {"compare_pat"}
// Dependencies: {}
pub (crate) fn compare_pat (a : & Pat , b : & Pat) -> bool { match (a , b) { (Pat :: Ident (a) , Pat :: Ident (b)) => a . ident == b . ident , (Pat :: Tuple (a) , Pat :: Tuple (b)) => a . elems == b . elems , (Pat :: TupleStruct (a) , Pat :: TupleStruct (b)) => a . path == b . path && a . elems == b . elems , (Pat :: Struct (a) , Pat :: Struct (b)) => a . path == b . path && a . fields == b . fields , _ => false , } }
};
}
