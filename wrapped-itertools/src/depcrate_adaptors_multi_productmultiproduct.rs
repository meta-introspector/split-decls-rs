// Generated macro for MultiProduct (struct)
macro_rules! Depcrate_adaptors_multi_productMultiProduct {
() => {
// Module: crate::adaptors::multi_product
// Provides: {"MultiProduct"}
// Dependencies: {}
# [derive (Clone)] # [doc = " An iterator adaptor that iterates over the cartesian product of"] # [doc = " multiple iterators of type `I`."] # [doc = ""] # [doc = " An iterator element type is `Vec<I::Item>`."] # [doc = ""] # [doc = " See [`.multi_cartesian_product()`](crate::Itertools::multi_cartesian_product)"] # [doc = " for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct MultiProduct < I > (State < MultiProductInner < I > >) where I : Iterator + Clone , I :: Item : Clone ;
};
}
