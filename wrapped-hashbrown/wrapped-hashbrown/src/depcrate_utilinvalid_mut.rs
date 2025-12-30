// Generated macro for invalid_mut (function)
macro_rules! Depcrate_utilinvalid_mut {
() => {
// Module: crate::util
// Provides: {"invalid_mut"}
// Dependencies: {}
# [inline (always)] # [allow (clippy :: useless_transmute)] pub (crate) fn invalid_mut < T > (addr : usize) -> * mut T { unsafe { core :: mem :: transmute (addr) } }
};
}
