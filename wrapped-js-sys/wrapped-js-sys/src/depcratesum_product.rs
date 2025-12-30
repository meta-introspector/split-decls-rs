// Generated macro for sum_product (macro)
macro_rules! Depcratesum_product {
() => {
// Module: crate
// Provides: {"sum_product"}
// Dependencies: {}
macro_rules ! sum_product { ($ ($ a : ident) *) => ($ (impl Sum for $ a { # [inline] fn sum < I : iter :: Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ a :: from (0) , | a , b | a + b ,) } } impl Product for $ a { # [inline] fn product < I : iter :: Iterator < Item = Self >> (iter : I) -> Self { iter . fold ($ a :: from (1) , | a , b | a * b ,) } } impl <'a > Sum <&'a $ a > for $ a { fn sum < I : iter :: Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ a :: from (0) , | a , b | a + b ,) } } impl <'a > Product <&'a $ a > for $ a { # [inline] fn product < I : iter :: Iterator < Item =&'a Self >> (iter : I) -> Self { iter . fold ($ a :: from (1) , | a , b | a * b ,) } }) *) }
};
}
