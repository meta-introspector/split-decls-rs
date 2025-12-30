// Generated macro for MultiProductIter (struct)
macro_rules! Depcrate_adaptors_multi_productMultiProductIter {
() => {
// Module: crate::adaptors::multi_product
// Provides: {"MultiProductIter"}
// Dependencies: {}
# [derive (Clone , Debug)] # [doc = " Holds the state of a single iterator within a `MultiProduct`."] struct MultiProductIter < I > where I : Iterator + Clone , I :: Item : Clone , { iter : I , iter_orig : I , }
};
}
