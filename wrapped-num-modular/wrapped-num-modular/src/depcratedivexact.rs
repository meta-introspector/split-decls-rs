// Generated macro for DivExact (trait)
macro_rules! DepcrateDivExact {
() => {
// Module: crate
// Provides: {"DivExact"}
// Dependencies: {}
# [doc = " Utility function for exact division, with precomputed helper values"] # [doc = ""] # [doc = " # Available Pre-computation types:"] # [doc = " - `()`: No pre-computation, the implementation relies on native integer division"] # [doc = " - [PreModInv]: With Pre-computed modular inverse"] pub trait DivExact < Rhs , Precompute > : Sized { type Output ; # [doc = " Check if d divides self with the help of the precomputation. If d divides self,"] # [doc = " then the quotient is returned."] fn div_exact (self , d : Rhs , pre : & Precompute) -> Option < Self :: Output > ; }
};
}
