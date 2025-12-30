// Generated macro for is_continuation (function)
macro_rules! Depcrate_utilis_continuation {
() => {
// Module: crate::util
// Provides: {"is_continuation"}
// Dependencies: {}
pub (super) const fn is_continuation (byte : u8) -> bool { byte & ! CONT_MASK == CONT_TAG }
};
}
