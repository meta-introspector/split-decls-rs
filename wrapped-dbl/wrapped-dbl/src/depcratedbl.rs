// Generated macro for Dbl (trait)
macro_rules! DepcrateDbl {
() => {
// Module: crate
// Provides: {"Dbl"}
// Dependencies: {}
# [doc = " Double and inverse double over `GF(2^n)` with the lexicographically first polynomial"] # [doc = " among the irreducible degree `n` polynomials having a minimum number of coefficients."] # [doc = ""] # [doc = " This trait is implemented using big-endian byte order for 64, 128 and 256 bit block sizes."] pub trait Dbl : sealed :: Sealed { # [doc = " Double block. (alternatively: multiply block by x)"] # [doc = ""] # [doc = " If most significant bit of the block equals to zero will return"] # [doc = " `block<<1`, otherwise `(block<<1)^C`, where `C` is the non-leading"] # [doc = " coefficients of the lexicographically first irreducible degree-b binary"] # [doc = " polynomial with the minimal number of ones."] # [must_use] fn dbl (self) -> Self ; # [doc = " Reverse double block. (alternatively: divide block by x)"] # [doc = ""] # [doc = " If least significant bit of the block equals to zero will return"] # [doc = " `block>>1`, otherwise `(block>>1)^(1<<n)^(C>>1)`"] # [must_use] fn inv_dbl (self) -> Self ; }
};
}
