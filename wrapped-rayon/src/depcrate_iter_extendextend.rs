// Generated macro for extend (macro)
macro_rules! Depcrate_iter_extendextend {
() => {
// Module: crate::iter::extend
// Provides: {"extend"}
// Dependencies: {}
# [doc = " Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in"] # [doc = " parallel, then extending the collection sequentially."] macro_rules ! extend { ($ self : ident , $ par_iter : ident) => { extend ! ($ self <- fast_collect ($ par_iter)) } ; ($ self : ident <- $ vecs : expr) => { match $ vecs { Either :: Left (vec) => $ self . extend (vec) , Either :: Right (list) => { for vec in list { $ self . extend (vec) ; } } } } ; }
};
}
