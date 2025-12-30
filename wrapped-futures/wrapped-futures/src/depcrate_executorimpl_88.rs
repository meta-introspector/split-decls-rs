// Generated macro for impl_88 (impl)
macro_rules! Depcrate_executorimpl_88 {
() => {
// Module: crate::executor
// Provides: {"impl_88"}
// Dependencies: {}
impl LimitState { fn new () -> LimitState { LimitState { count : Cell :: new (0) , deferred : RefCell :: new (Vec :: new ()) , } } fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static { match self . count . get () { 0 => { self . count . set (1) ; f () ; loop { let cb = self . deferred . borrow_mut () . pop () ; match cb { Some (f) => f . call () , None => break , } } self . count . set (0) ; } n if n < LIMIT => { self . count . set (n + 1) ; f () ; self . count . set (n) ; } _ => self . deferred . borrow_mut () . push (Box :: new (f)) , } } }
};
}
