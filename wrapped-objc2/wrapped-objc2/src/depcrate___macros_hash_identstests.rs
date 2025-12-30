// Generated macro for tests (module)
macro_rules! Depcrate___macros_hash_identstests {
() => {
// Module: crate::__macros::hash_idents
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (any (feature = "unstable-static-sel" , feature = "unstable-static-class"))] mod tests { # [test] fn hash_idents_different () { assert_ne ! (__hash_idents ! (abc) , __hash_idents ! (def)) ; } # [test] fn hash_idents_same_no_equal () { assert_ne ! (__hash_idents ! (abc) , __hash_idents ! (abc)) ; assert_ne ! (__hash_idents ! (abc def ghi) , __hash_idents ! (abc def ghi)) ; } # [test] fn hash_idents_exact_same_ident () { macro_rules ! x { ($ x : ident) => { (__hash_idents ! ($ x) , __hash_idents ! ($ x)) } ; } let (ident1 , ident2) = x ! (abc) ; assert_eq ! (ident1 , ident2) ; } }
};
}
