// Generated macro for WithSmallOrderMulGroup (trait)
macro_rules! DepcrateWithSmallOrderMulGroup {
() => {
// Module: crate
// Provides: {"WithSmallOrderMulGroup"}
// Dependencies: {}
# [doc = " The subset of prime-order fields such that `(modulus - 1)` is divisible by `N`."] # [doc = ""] # [doc = " If `N` is prime, there will be `N - 1` valid choices of [`Self::ZETA`]. Similarly to"] # [doc = " [`PrimeField::MULTIPLICATIVE_GENERATOR`], the specific choice does not matter, as long"] # [doc = " as the choice is consistent across all uses of the field."] pub trait WithSmallOrderMulGroup < const N : u8 > : PrimeField { # [doc = " A field element of small multiplicative order $N$."] # [doc = ""] # [doc = " The presence of this element allows you to perform (certain types of)"] # [doc = " endomorphisms on some elliptic curves."] # [doc = ""] # [doc = " It can be calculated using [SageMath] as"] # [doc = " `GF(modulus).primitive_element() ^ ((modulus - 1) // N)`."] # [doc = " Choosing the element of order $N$ that is smallest, when considered"] # [doc = " as an integer, may help to ensure consistency."] # [doc = ""] # [doc = " [SageMath]: https://www.sagemath.org/"] const ZETA : Self ; }
};
}
