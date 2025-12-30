// Generated macro for Offset (trait)
macro_rules! Depcrate_offsetOffset {
() => {
// Module: crate::offset
// Provides: {"Offset"}
// Dependencies: {}
# [doc = " The offset from the local time to UTC."] pub trait Offset : Sized + Clone + fmt :: Debug { # [doc = " Returns the fixed offset from UTC to the local time stored."] fn fix (& self) -> FixedOffset ; }
};
}
