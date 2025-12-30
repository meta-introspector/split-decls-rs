// Generated macro for invalid_mut (function)
macro_rules! Depcrateinvalid_mut {
() => {
// Module: crate
// Provides: {"invalid_mut"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [inline (always)] fn invalid_mut < T > (addr : usize) -> * mut T { # [allow (clippy :: useless_transmute , clippy :: transmutes_expressible_as_ptr_casts)] unsafe { core :: mem :: transmute (addr) } }
};
}
