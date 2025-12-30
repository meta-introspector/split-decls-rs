// Generated macro for FixedMersenne (struct)
macro_rules! Depcrate_mersenneFixedMersenne {
() => {
// Module: crate::mersenne
// Provides: {"FixedMersenne"}
// Dependencies: {}
# [doc = " A modular reducer for (pseudo) Mersenne numbers `2^P - K` as modulus. It supports `P` up to 127 and `K < 2^(P-1)`"] # [doc = ""] # [doc = " The `P` is limited to 127 so that it's not necessary to check overflow. This limit won't be a problem for any"] # [doc = " Mersenne primes within the range of [umax] (i.e. [u128])."] # [derive (Debug , Clone , Copy)] pub struct FixedMersenne < const P : u8 , const K : umax > () ;
};
}
