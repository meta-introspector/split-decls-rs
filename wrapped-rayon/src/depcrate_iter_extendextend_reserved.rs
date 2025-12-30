// Generated macro for extend_reserved (macro)
macro_rules! Depcrate_iter_extendextend_reserved {
() => {
// Module: crate::iter::extend
// Provides: {"extend_reserved"}
// Dependencies: {}
macro_rules ! extend_reserved { ($ self : ident , $ par_iter : ident , $ len : ident) => { let vecs = fast_collect ($ par_iter) ; $ self . reserve ($ len (& vecs)) ; extend ! ($ self <- vecs) } ; ($ self : ident , $ par_iter : ident) => { extend_reserved ! ($ self , $ par_iter , len) } ; }
};
}
