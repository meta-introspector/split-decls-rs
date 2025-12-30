// Generated macro for Closure (struct)
macro_rules! Depcrate_bridge_closureClosure {
() => {
// Module: crate::bridge::closure
// Provides: {"Closure"}
// Dependencies: {}
# [repr (C)] pub (super) struct Closure < 'a , A , R > { call : unsafe extern "C" fn (* mut Env , A) -> R , env : * mut Env , _marker : PhantomData < * mut & 'a mut () > , }
};
}
