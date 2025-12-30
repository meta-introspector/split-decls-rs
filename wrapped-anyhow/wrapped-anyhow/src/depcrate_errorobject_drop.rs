// Generated macro for object_drop (function)
macro_rules! Depcrate_errorobject_drop {
() => {
// Module: crate::error
// Provides: {"object_drop"}
// Dependencies: {}
unsafe fn object_drop < E > (e : Own < ErrorImpl >) { let unerased_own = e . cast :: < ErrorImpl < E > > () ; drop (unsafe { unerased_own . boxed () }) ; }
};
}
