// Generated macro for impl_98 (impl)
macro_rules! Depcrate_interact_contextimpl_98 {
() => {
// Module: crate::interact::context
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , Session , Input , Output , State > Context < 'a , Session , Input , Output , State > { # [doc = " Creates a new [`Context`] structure."] pub fn new (session : & 'a mut Session , input : & 'a mut Input , output : & 'a mut Output , state : & 'a mut State , buf : & 'a [u8] , eof : bool ,) -> Self { Self { session , input , output , buf , eof , state , } } }
};
}
