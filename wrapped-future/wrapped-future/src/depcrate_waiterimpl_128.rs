// Generated macro for impl_128 (impl)
macro_rules! Depcrate_waiterimpl_128 {
() => {
// Module: crate::waiter
// Provides: {"impl_128"}
// Dependencies: {}
impl Waiter { pub fn new () -> crate :: Result < (Self , WaiterSignaler) > { unsafe { let handle = CreateEventW (core :: ptr :: null () , 1 , 0 , core :: ptr :: null ()) ; if handle . is_null () { Err (crate :: Error :: from_thread ()) } else { Ok ((Self (handle) , WaiterSignaler (handle))) } } } }
};
}
