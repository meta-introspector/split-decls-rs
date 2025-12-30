// Generated macro for __private (module)
macro_rules! Depcrate_notify__private {
() => {
// Module: crate::notify
// Provides: {"__private"}
// Dependencies: {}
mod __private { # [doc = " Make sure the NotificationPrivate trait can't be implemented outside of this crate."] # [doc (hidden)] # [derive (Debug)] pub struct Internal (()) ; impl Internal { pub (crate) fn new () -> Self { Self (()) } } # [doc (hidden)] pub trait Sealed { } impl < N : super :: NotificationPrivate + ? Sized > Sealed for N { } }
};
}
