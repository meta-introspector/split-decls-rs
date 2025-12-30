// Generated macro for sieve_not_all_primes (function)
macro_rules! Depcrate_testssieve_not_all_primes {
() => {
// Module: crate::tests
// Provides: {"sieve_not_all_primes"}
// Dependencies: {}
# [test] # [should_panic] fn sieve_not_all_primes () { fn prop_prime_iff_in_the_sieve (n : u8) -> bool { let n = n as usize ; sieve (n) == (0 ..= n) . filter (| & i | is_prime (i)) . collect :: < Vec < _ > > () } quickcheck (prop_prime_iff_in_the_sieve as fn (u8) -> bool) ; }
};
}
