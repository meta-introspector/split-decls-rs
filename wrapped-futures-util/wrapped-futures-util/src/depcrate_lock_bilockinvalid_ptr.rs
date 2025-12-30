// Generated macro for invalid_ptr (function)
macro_rules! Depcrate_lock_bilockinvalid_ptr {
() => {
// Module: crate::lock::bilock
// Provides: {"invalid_ptr"}
// Dependencies: {}
# [allow (clippy :: useless_transmute)] # [inline] fn invalid_ptr < T > (addr : usize) -> * mut T { unsafe { core :: mem :: transmute (addr) } }
};
}
