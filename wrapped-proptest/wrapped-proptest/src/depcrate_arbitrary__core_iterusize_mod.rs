// Generated macro for usize_mod (macro)
macro_rules! Depcrate_arbitrary__core_iterusize_mod {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"usize_mod"}
// Dependencies: {}
macro_rules ! usize_mod { ($ type : ident , $ mapper : ident) => { arbitrary ! ([A : Arbitrary + Iterator] $ type < A >, SMapped < (A , usize) , Self >, A :: Parameters ; a => static_map (any_with ::< (A , usize) > (product_pack ! [a , ()]) , | (a , b) | a .$ mapper (b))) ; lift1 ! ([Iterator] $ type < A >; base => (base , any ::< usize > ()) . prop_map (| (a , b) | a .$ mapper (b))) ; } ; }
};
}
