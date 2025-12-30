// Generated macro for MultiProductInner (struct)
macro_rules! Depcrate_adaptors_multi_productMultiProductInner {
() => {
// Module: crate::adaptors::multi_product
// Provides: {"MultiProductInner"}
// Dependencies: {}
# [derive (Clone)] # [doc = " Internals for `MultiProduct`."] struct MultiProductInner < I > where I : Iterator + Clone , I :: Item : Clone , { # [doc = " Holds the iterators."] iters : Vec < MultiProductIter < I > > , # [doc = " Not populated at the beginning then it holds the current item of each iterator."] cur : CurrentItems < Vec < I :: Item > > , }
};
}
