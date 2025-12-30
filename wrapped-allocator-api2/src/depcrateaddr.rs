// Generated macro for addr (function)
macro_rules! Depcrateaddr {
() => {
// Module: crate
// Provides: {"addr"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [inline (always)] fn addr < T > (x : * const T) -> usize { # [allow (clippy :: useless_transmute , clippy :: transmutes_expressible_as_ptr_casts)] unsafe { core :: mem :: transmute (x) } }
};
}
