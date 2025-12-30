// Generated macro for iproduct (macro)
macro_rules! Depcrateiproduct {
() => {
// Module: crate
// Provides: {"iproduct"}
// Dependencies: {}
# [macro_export] # [doc = " Create an iterator over the “cartesian product” of iterators."] # [doc = ""] # [doc = " Iterator element type is like `(A, B, ..., E)` if formed"] # [doc = " from iterators `(I, J, ..., M)` with element types `I::Item = A`, `J::Item = B`, etc."] # [doc = ""] # [doc = " ```"] # [doc = " # use itertools::iproduct;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " // Iterate over the coordinates of a 4 x 4 x 4 grid"] # [doc = " // from (0, 0, 0), (0, 0, 1), .., (0, 1, 0), (0, 1, 1), .. etc until (3, 3, 3)"] # [doc = " for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {"] # [doc = "    // .."] # [doc = "    # let _ = (i, j, k);"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] macro_rules ! iproduct { (@ flatten $ I : expr ,) => ($ I) ; (@ flatten $ I : expr , $ J : expr , $ ($ K : expr ,) *) => ($ crate :: iproduct ! (@ flatten $ crate :: cons_tuples ($ crate :: iproduct ! ($ I , $ J)) , $ ($ K ,) *)) ; () => ($ crate :: __std_iter :: once (())) ; ($ I : expr $ (,) ?) => ($ crate :: __std_iter :: Iterator :: map ($ crate :: __std_iter :: IntoIterator :: into_iter ($ I) , | elt | (elt ,))) ; ($ I : expr , $ J : expr $ (,) ?) => ($ crate :: Itertools :: cartesian_product ($ crate :: __std_iter :: IntoIterator :: into_iter ($ I) , $ crate :: __std_iter :: IntoIterator :: into_iter ($ J) ,)) ; ($ I : expr , $ J : expr , $ ($ K : expr) ,+ $ (,) ?) => ($ crate :: iproduct ! (@ flatten $ crate :: iproduct ! ($ I , $ J) , $ ($ K ,) +)) ; }
};
}
