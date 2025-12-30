// Generated macro for add (function)
macro_rules! Depcrate_shims_x86_shaadd {
() => {
// Module: crate::shims::x86::sha
// Provides: {"add"}
// Dependencies: {}
# [inline (always)] fn add (a : [u32 ; 4] , b : [u32 ; 4]) -> [u32 ; 4] { [a [0] . wrapping_add (b [0]) , a [1] . wrapping_add (b [1]) , a [2] . wrapping_add (b [2]) , a [3] . wrapping_add (b [3]) ,] }
};
}
