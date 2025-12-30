// Generated macro for permutation_trait (macro)
macro_rules! Depcrate_branchpermutation_trait {
() => {
// Module: crate::branch
// Provides: {"permutation_trait"}
// Dependencies: {}
macro_rules ! permutation_trait (($ name1 : ident $ ty1 : ident $ item1 : ident $ name2 : ident $ ty2 : ident $ item2 : ident $ ($ name3 : ident $ ty3 : ident $ item3 : ident) *) => (permutation_trait ! (__impl $ name1 $ ty1 $ item1 , $ name2 $ ty2 $ item2 ; $ ($ name3 $ ty3 $ item3) *) ;) ; (__impl $ ($ name : ident $ ty : ident $ item : ident) ,+; $ name1 : ident $ ty1 : ident $ item1 : ident $ ($ name2 : ident $ ty2 : ident $ item2 : ident) *) => (permutation_trait_impl ! ($ ($ name $ ty $ item) ,+) ; permutation_trait ! (__impl $ ($ name $ ty $ item) ,+ , $ name1 $ ty1 $ item1 ; $ ($ name2 $ ty2 $ item2) *) ;) ; (__impl $ ($ name : ident $ ty : ident $ item : ident) ,+;) => (permutation_trait_impl ! ($ ($ name $ ty $ item) ,+) ;) ;) ;
};
}
