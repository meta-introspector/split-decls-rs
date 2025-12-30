// Generated macro for context_chain_drop_rest (function)
macro_rules! Depcrate_errorcontext_chain_drop_rest {
() => {
// Module: crate::error
// Provides: {"context_chain_drop_rest"}
// Dependencies: {}
unsafe fn context_chain_drop_rest < C > (e : Own < ErrorImpl > , target : TypeId) where C : 'static , { if TypeId :: of :: < C > () == target { let unerased_own = e . cast :: < ErrorImpl < ContextError < ManuallyDrop < C > , Error > > > () ; drop (unsafe { unerased_own . boxed () }) ; } else { let unerased_own = e . cast :: < ErrorImpl < ContextError < C , ManuallyDrop < Error > > > > () ; let unerased = unsafe { unerased_own . boxed () } ; let inner = unerased . _object . error . inner ; drop (unerased) ; let vtable = unsafe { vtable (inner . ptr) } ; unsafe { (vtable . object_drop_rest) (inner , target) } ; } }
};
}
