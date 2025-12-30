// Generated macro for impl_103 (impl)
macro_rules! Depcrateimpl_103 {
() => {
// Module: crate
// Provides: {"impl_103"}
// Dependencies: {}
impl IterState { fn new () -> IterState { IterState { done : false } } fn next (& mut self , js : & Iterator) -> Option < Result < JsValue , JsValue > > { if self . done { return None ; } let next = match js . next () { Ok (val) => val , Err (e) => { self . done = true ; return Some (Err (e)) ; } } ; if next . done () { self . done = true ; None } else { Some (Ok (next . value ())) } } }
};
}
