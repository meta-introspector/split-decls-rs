// Generated macro for permutation_trait_inner (macro)
macro_rules! Depcrate_branchpermutation_trait_inner {
() => {
// Module: crate::branch
// Provides: {"permutation_trait_inner"}
// Dependencies: {}
macro_rules ! permutation_trait_inner (($ it : tt , $ self : expr , $ input : ident , $ res : expr , $ err : expr , $ head : ident $ ($ item : ident) *) => (if !$ head { match $ self . parser .$ it . process ::< OM > ($ input . clone ()) { Ok ((i , o)) => { $ input = i ; $ res = OM :: Output :: combine ($ res , o , | mut res , o | { res .$ it = Some (o) ; res }) ; $ head = true ; continue ; } Err (Err :: Error (e)) => { $ err = Some (match $ err { None => e , Some (err) => OM :: Error :: combine (err , e , | err , e | err . or (e)) }) ; } Err (e) => return Err (e) , } ; } succ ! ($ it , permutation_trait_inner ! ($ self , $ input , $ res , $ err , $ ($ item) *)) ;) ; ($ it : tt , $ self : expr , $ input : ident , $ res : expr , $ err : expr ,) => () ;) ;
};
}
