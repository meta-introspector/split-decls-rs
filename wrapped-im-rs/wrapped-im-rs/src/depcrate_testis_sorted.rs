// Generated macro for is_sorted (function)
macro_rules! Depcrate_testis_sorted {
() => {
// Module: crate::test
// Provides: {"is_sorted"}
// Dependencies: {}
pub (crate) fn is_sorted < A , I > (l : I) -> bool where I : IntoIterator < Item = A > , A : Ord , { let mut it = l . into_iter () . peekable () ; loop { match (it . next () , it . peek ()) { (_ , None) => return true , (Some (ref a) , Some (b)) if a > b => return false , _ => () , } } }
};
}
