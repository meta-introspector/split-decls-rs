// Generated macro for unpack (function)
macro_rules! Depcrate_state_definitionsunpack {
() => {
// Module: crate::state::definitions
// Provides: {"unpack"}
// Dependencies: {}
# [doc = " Unpack a u8 into a State and Action"] # [doc = ""] # [doc = " The implementation of this assumes that there are *precisely* 16 variants for both Action and"] # [doc = " State. Furthermore, it assumes that the enums are tag-only; that is, there is no data in any"] # [doc = " variant."] # [doc = ""] # [doc = " Bad things will happen if those invariants are violated."] # [inline (always)] pub (crate) const fn unpack (delta : u8) -> (State , Action) { unsafe { (mem :: transmute :: < u8 , State > (delta & 0x0f) , mem :: transmute :: < u8 , Action > (delta >> 4) ,) } }
};
}
