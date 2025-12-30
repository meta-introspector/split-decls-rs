// Generated macro for waitable_unregister (function)
macro_rules! Depcrate_rt_async_supportwaitable_unregister {
() => {
// Module: crate::rt::async_support
// Provides: {"waitable_unregister"}
// Dependencies: {}
unsafe extern "C" fn waitable_unregister (ptr : * mut c_void , waitable : u32) -> * mut c_void { let ptr = ptr . cast :: < FutureState < 'static > > () ; assert ! (! ptr . is_null ()) ; (* ptr) . remove_waitable (waitable) ; match (* ptr) . waitables . remove (& waitable) { Some ((prev , _)) => prev , None => ptr :: null_mut () , } }
};
}
