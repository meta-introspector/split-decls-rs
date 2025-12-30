// Generated macro for NSAutoreleasePool (trait)
macro_rules! Depcrate_foundationNSAutoreleasePool {
() => {
// Module: crate::foundation
// Provides: {"NSAutoreleasePool"}
// Dependencies: {}
pub trait NSAutoreleasePool : Sized { unsafe fn new (_ : Self) -> id { msg_send ! [class ! (NSAutoreleasePool) , new] } unsafe fn autorelease (self) -> Self ; unsafe fn drain (self) ; }
};
}
