// Generated macro for Finder (struct)
macro_rules! Depcrate_arch_all_rabinkarpFinder {
() => {
// Module: crate::arch::all::rabinkarp
// Provides: {"Finder"}
// Dependencies: {}
# [doc = " A forward substring searcher using the Rabin-Karp algorithm."] # [doc = ""] # [doc = " Note that, as a lower level API, a `Finder` does not have access to the"] # [doc = " needle it was constructed with. For this reason, executing a search"] # [doc = " with a `Finder` requires passing both the needle and the haystack,"] # [doc = " where the needle is exactly equivalent to the one given to the `Finder`"] # [doc = " at construction time. This design was chosen so that callers can have"] # [doc = " more precise control over where and how many times a needle is stored."] # [doc = " For example, in cases where Rabin-Karp is just one of several possible"] # [doc = " substring search algorithms."] # [derive (Clone , Debug)] pub struct Finder { # [doc = " The actual hash."] hash : Hash , # [doc = " The factor needed to multiply a byte by in order to subtract it from"] # [doc = " the hash. It is defined to be 2^(n-1) (using wrapping exponentiation),"] # [doc = " where n is the length of the needle. This is how we \"remove\" a byte"] # [doc = " from the hash once the hash window rolls past it."] hash_2pow : u32 , }
};
}
