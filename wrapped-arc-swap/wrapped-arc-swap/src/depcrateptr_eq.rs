// Generated macro for ptr_eq (function)
macro_rules! Depcrateptr_eq {
() => {
// Module: crate
// Provides: {"ptr_eq"}
// Dependencies: {}
# [doc = " Comparison of two pointer-like things."] # [allow (clippy :: needless_pass_by_value)] fn ptr_eq < Base , A , B > (a : A , b : B) -> bool where A : AsRaw < Base > , B : AsRaw < Base > , { let a = a . as_raw () ; let b = b . as_raw () ; ptr :: eq (a , b) }
};
}
