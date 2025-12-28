macro_rules! checked_binomial {
    () => {
        pub (crate) fn checked_binomial (mut n : usize , mut k : usize) -> Option < usize > { if n < k { return Some (0) ; } k = (n - k) . min (k) ; let mut c = 1 ; for i in 1 ..= k { c = (c / i) . checked_mul (n) ? . checked_add ((c % i) . checked_mul (n) ? / i) ? ; n -= 1 ; } Some (c) }
    };
}

checked_binomial!();