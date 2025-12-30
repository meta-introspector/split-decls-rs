// Generated macro for waitable_register (function)
macro_rules! Depcrate_rt_async_supportwaitable_register {
() => {
// Module: crate::rt::async_support
// Provides: {"waitable_register"}
// Dependencies: {}
unsafe extern "C" fn waitable_register (ptr : * mut c_void , waitable : u32 , callback : unsafe extern "C" fn (* mut c_void , u32) , callback_ptr : * mut c_void ,) -> * mut c_void { let ptr = ptr . cast :: < FutureState < 'static > > () ; assert ! (! ptr . is_null ()) ; (* ptr) . add_waitable (waitable) ; match (* ptr) . waitables . insert (waitable , (callback_ptr , callback)) { Some ((prev , _)) => prev , None => ptr :: null_mut () , } }
};
}
