// Generated macro for pat_is_self (function)
macro_rules! Depcratepat_is_self {
() => {
// Module: crate
// Provides: {"pat_is_self"}
// Dependencies: {}
# [doc = " Determine if this Pat is any kind of `self` binding"] fn pat_is_self (pat : & Pat) -> bool { if let Pat :: Ident (pi) = pat { pi . ident == "self" } else { false } }
};
}
