// Generated macro for nest (macro)
macro_rules! Depcrate_iter_multizipnest {
() => {
// Module: crate::iter::multizip
// Provides: {"nest"}
// Dependencies: {}
macro_rules ! nest { ($ A : tt , $ B : tt , $ C : tt , $ D : tt , $ ($ X : tt) ,+) => { (nest ! ($ A , $ B , $ C , $ D) , nest ! ($ ($ X) ,+)) } ; ($ A : tt , $ B : tt , $ ($ X : tt) ,+) => { (($ A , $ B) , nest ! ($ ($ X) ,+)) } ; ($ A : tt , $ B : tt) => { ($ A , $ B) } ; ($ A : tt) => { $ A } ; }
};
}
