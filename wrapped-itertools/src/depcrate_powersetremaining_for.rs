// Generated macro for remaining_for (function)
macro_rules! Depcrate_powersetremaining_for {
() => {
// Module: crate::powerset
// Provides: {"remaining_for"}
// Dependencies: {}
fn remaining_for (n : usize , k : usize) -> Option < usize > { (k + 1 ..= n) . try_fold (0usize , | sum , i | sum . checked_add (checked_binomial (n , i) ?)) }
};
}
