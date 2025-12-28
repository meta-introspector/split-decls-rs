macro_rules! sieve_not_all_primes {
    () => {
        # [test] # [should_panic] fn sieve_not_all_primes () { fn prop_prime_iff_in_the_sieve (n : u8) -> bool { let n = n as usize ; sieve (n) == (0 ..= n) . filter (| & i | is_prime (i)) . collect :: < Vec < _ > > () } quickcheck (prop_prime_iff_in_the_sieve as fn (u8) -> bool) ; }
    };
}

sieve_not_all_primes!()