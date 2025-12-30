// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [no_mangle] fn main (_argc : isize , _argv : * const * const u8) -> isize { let mut h : ahash :: AHasher = Default :: default () ; 42_i32 . hash (& mut h) ; return h . finish () as isize ; }
};
}
