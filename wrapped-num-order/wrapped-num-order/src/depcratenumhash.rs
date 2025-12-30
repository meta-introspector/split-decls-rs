// Generated macro for NumHash (trait)
macro_rules! DepcrateNumHash {
() => {
// Module: crate
// Provides: {"NumHash"}
// Dependencies: {}
# [doc = " Consistent hash implementation among different numeric types."] # [doc = ""] # [doc = " It's ensured that if `a.num_eq(b)`, then `a` and `b` will result in the same hash. Although the other direction is"] # [doc = " not ensured because it's infeasible, the hash function is still designed to be as sparse as possible."] pub trait NumHash { # [doc = " Consistent [Hash::hash][core::hash::Hash::hash] on different numeric types."] # [doc = ""] # [doc = " This function will ensures if `a.num_eq(b)`, then `a.num_hash()` and `b.num_hash()` manipulate the state in the same way."] fn num_hash < H : Hasher > (& self , state : & mut H) ; }
};
}
