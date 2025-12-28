macro_rules! sieve_not_prime {
    () => {
        # [test] # [should_panic] fn sieve_not_prime () { fn prop_all_prime (n : u8) -> bool { sieve (n as usize) . into_iter () . all (is_prime) } quickcheck (prop_all_prime as fn (u8) -> bool) ; }
    };
}

sieve_not_prime!();