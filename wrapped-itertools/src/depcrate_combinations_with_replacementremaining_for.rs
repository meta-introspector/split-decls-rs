// Generated macro for remaining_for (function)
macro_rules! Depcrate_combinations_with_replacementremaining_for {
() => {
// Module: crate::combinations_with_replacement
// Provides: {"remaining_for"}
// Dependencies: {}
# [doc = " For a given size `n`, return the count of remaining combinations with replacement or None if it would overflow."] fn remaining_for (n : usize , first : bool , indices : & [usize]) -> Option < usize > { let count = | n : usize , k : usize | { let positions = if n == 0 { k . saturating_sub (1) } else { (n - 1) . checked_add (k) ? } ; checked_binomial (positions , k) } ; let k = indices . len () ; if first { count (n , k) } else { indices . iter () . enumerate () . try_fold (0usize , | sum , (i , n0) | { sum . checked_add (count (n - 1 - * n0 , k - i) ?) }) } }
};
}
