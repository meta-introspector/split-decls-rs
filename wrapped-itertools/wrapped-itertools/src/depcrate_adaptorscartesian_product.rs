// Generated macro for cartesian_product (function)
macro_rules! Depcrate_adaptorscartesian_product {
() => {
// Module: crate::adaptors
// Provides: {"cartesian_product"}
// Dependencies: {}
# [doc = " Create a new cartesian product iterator"] # [doc = ""] # [doc = " Iterator element type is `(I::Item, J::Item)`."] pub fn cartesian_product < I , J > (i : I , j : J) -> Product < I , J > where I : Iterator , J : Clone + Iterator , I :: Item : Clone , { Product { a_cur : None , a : i , b : j . clone () , b_orig : j , } }
};
}
