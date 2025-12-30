// Generated macro for unsafe_unwrap_unchecked (macro)
macro_rules! Depcrateunsafe_unwrap_unchecked {
() => {
// Module: crate
// Provides: {"unsafe_unwrap_unchecked"}
// Dependencies: {}
# [doc = " `Option::unwrap_unchecked`, but usable in `const` contexts."] macro_rules ! unsafe_unwrap_unchecked { ($ e : expr) => { { let opt = $ e ; debug_assert ! (opt . is_some ()) ; match $ e { Some (value) => value , None => core :: hint :: unreachable_unchecked () , } } } ; }
};
}
