// Generated macro for object_drop_front (function)
macro_rules! Depcrate_errorobject_drop_front {
() => {
// Module: crate::error
// Provides: {"object_drop_front"}
// Dependencies: {}
unsafe fn object_drop_front < E > (e : Own < ErrorImpl > , target : TypeId) { let _ = target ; let unerased_own = e . cast :: < ErrorImpl < ManuallyDrop < E > > > () ; drop (unsafe { unerased_own . boxed () }) ; }
};
}
