// Generated macro for remaining_for (function)
macro_rules! Depcrate_combinationsremaining_for {
() => {
// Module: crate::combinations
// Provides: {"remaining_for"}
// Dependencies: {}
# [doc = " For a given size `n`, return the count of remaining combinations or None if it would overflow."] fn remaining_for (n : usize , first : bool , indices : & [usize]) -> Option < usize > { let k = indices . len () ; if n < k { Some (0) } else if first { checked_binomial (n , k) } else { indices . iter () . enumerate () . try_fold (0usize , | sum , (i , n0) | { sum . checked_add (checked_binomial (n - 1 - * n0 , k - i) ?) }) } }
};
}
