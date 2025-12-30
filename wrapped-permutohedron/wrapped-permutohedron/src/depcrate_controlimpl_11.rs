// Generated macro for impl_11 (impl)
macro_rules! Depcrate_controlimpl_11 {
() => {
// Module: crate::control
// Provides: {"impl_11"}
// Dependencies: {}
impl < B > Control < B > { # [must_use] pub const fn breaking () -> Control < () > { Control :: Break (()) } # [doc = " Get the value in `Control::Break(_)`, if present."] pub fn break_value (self) -> Option < B > { match self { Self :: Continue => None , Self :: Break (b) => Some (b) , } } }
};
}
