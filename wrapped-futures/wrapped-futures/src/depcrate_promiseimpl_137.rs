// Generated macro for impl_137 (impl)
macro_rules! Depcrate_promiseimpl_137 {
() => {
// Module: crate::promise
// Provides: {"impl_137"}
// Dependencies: {}
impl < T > Complete < T > where T : Send + 'static , { # [doc = " Completes this promise with a successful result."] # [doc = ""] # [doc = " This function will consume `self` and indicate to the other end, the"] # [doc = " `Promise`, that the error provided is the result of the computation this"] # [doc = " represents."] pub fn complete (mut self , t : T) { self . completed = true ; self . send (Some (t)) } fn send (& mut self , t : Option < T >) { if let Err (e) = self . inner . slot . try_produce (t) { self . inner . slot . on_empty (| slot | { slot . try_produce (e . into_inner ()) . ok () . expect ("advertised as empty but wasn't") ; }) ; } } }
};
}
